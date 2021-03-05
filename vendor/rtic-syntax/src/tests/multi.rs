use std::collections::BTreeSet;

use quote::quote;

use crate::{analyze::Location, Settings};

#[test]
fn ast_extern_interrupt_core() {
    let (app, _analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                extern "C" {
                    #[core = 0]
                    fn a();

                    #[core = 1]
                    fn a();

                    #[core = 1]
                    fn b();
                }
            };
        ),
        Settings {
            parse_cores: true,
            parse_extern_interrupt: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let interrupts0 = &app.extern_interrupts[&0];
    assert_eq!(interrupts0.len(), 1);
    let mut interrupts = interrupts0.iter();
    let (name, interrupt) = interrupts.next().unwrap();
    assert_eq!(name.to_string(), "a");
    assert!(interrupt.attrs.is_empty());

    let interrupts1 = &app.extern_interrupts[&1];
    assert_eq!(interrupts1.len(), 2);
    let mut interrupts = interrupts1.iter();
    let (name, interrupt) = interrupts.next().unwrap();
    assert_eq!(name.to_string(), "a");
    assert!(interrupt.attrs.is_empty());
    let (name, interrupt) = interrupts.next().unwrap();
    assert_eq!(name.to_string(), "b");
    assert!(interrupt.attrs.is_empty());
}

#[test]
fn unused_resource() {
    // this shouldn't crash the analysis
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    #[init(0)]
                    x: i32,
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    // `X` shouldn't be listed in `locations`
    assert!(analysis.locations.is_empty());
}

#[test]
fn unused_task() {
    // this shouldn't crash the analysis
    let (_app, _analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                #[task(core = 1)]
                fn foo(_: foo::Context) {}
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();
}

#[test]
fn late_resources_split() {
    // split initialization of late resources
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    x: i32,
                    y: i32,
                }

                #[init(core = 0, late = [x])]
                fn init(_: init::Context) -> init::LateResources {
                    ..
                }

                #[init(core = 1)]
                fn init(_: init::Context) -> init::LateResources {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let late0 = &analysis.late_resources[&0];
    let late1 = &analysis.late_resources[&1];
    assert_eq!(late0.len(), 1);
    assert_eq!(late0.iter().next().unwrap().to_string(), "x");

    assert_eq!(late1.len(), 1);
    assert_eq!(late1.iter().next().unwrap().to_string(), "y");
}

#[test]
fn late_resources() {
    // one core initializes all late resources
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    x: i32,
                }

                #[init(core = 0)]
                fn init(_: init::Context) -> init::LateResources {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let late = &analysis.late_resources[&0];
    assert_eq!(late.len(), 1);
}

#[test]
fn location_resource() {
    // early resources are located on the cores they are accessed from
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    #[init(0)]
                    x: i32,
                    #[init(0)]
                    y: i32,
                }

                #[task(core = 0, resources = [x])]
                fn foo(_: foo::Context) {}

                #[task(core = 1, resources = [y])]
                fn bar(_: bar::Context) {}
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    assert_eq!(analysis.locations.len(), 2);

    let (name, loc) = analysis.locations.get_index(0).unwrap();
    assert_eq!(name.to_string(), "x");
    assert_eq!(
        *loc,
        Location::Owned {
            core: 0,
            cross_initialized: false
        }
    );

    let (name, loc) = analysis.locations.get_index(1).unwrap();
    assert_eq!(name.to_string(), "y");
    assert_eq!(
        *loc,
        Location::Owned {
            core: 1,
            cross_initialized: false
        }
    );
}

#[test]
fn initialization_barrier() {
    // core #0 initializes core #1's resource: a barrier is needed
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    x: i32,
                }

                #[init(core = 0)]
                fn init(_: init::Context) -> init::LateResources {
                    ..
                }

                #[idle(core = 1, resources = [x])]
                fn idle(_: idle::Context) -> ! {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let barriers = &analysis.initialization_barriers[&1];
    assert_eq!(barriers.len(), 1);
    assert_eq!(*barriers.iter().next().unwrap(), 0);
}

#[test]
fn send_spawn() {
    // cross-core message passing needs a `Send` bound regardless of priorities
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                #[task(core = 0, spawn = [bar])]
                fn foo(_: foo::Context) {
                    ..
                }

                #[task(core = 1)]
                fn bar(_: bar::Context, x: X) {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    assert_eq!(analysis.send_types[&1].len(), 1);
    let ty = analysis.send_types[&1].iter().next().unwrap();
    assert_eq!(quote!(#ty).to_string(), "X");
}

#[test]
fn send_schedule() {
    // cross-core message passing needs a `Send` bound regardless of priorities
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                #[task(core = 0, schedule = [bar])]
                fn foo(_: foo::Context) {
                    ..
                }

                #[task(core = 1)]
                fn bar(_: bar::Context, x: X) {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            parse_schedule: true,
            ..Settings::default()
        },
    )
    .unwrap();

    assert_eq!(analysis.send_types[&1].len(), 1);
    let ty = analysis.send_types[&1].iter().next().unwrap();
    assert_eq!(quote!(#ty).to_string(), "X");
}

#[test]
fn sync() {
    // `static` resources shared between cores need to be `Sync` regardless of priorities
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    #[init(0)]
                    x: i32,
                }

                #[idle(core = 0, resources = [&x])]
                fn idle(_: idle::Context) -> ! {
                    ..
                }

                #[idle(core = 1, resources = [&x])]
                fn idle(_: idle::Context) -> ! {
                    ..
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let ty = analysis.sync_types[&0].iter().next().unwrap();
    assert_eq!(quote!(#ty).to_string(), "i32");

    let ty = analysis.sync_types[&1].iter().next().unwrap();
    assert_eq!(quote!(#ty).to_string(), "i32");
}

#[test]
fn timer_queue() {
    // when cross scheduling, the timer handler needs to run at the highest priority in its core
    let (_app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                #[task(core = 0, schedule = [bar, baz])]
                fn foo(_: foo::Context) {}

                #[task(core = 0, priority = 2)]
                fn bar(_: bar::Context) {}

                #[task(core = 1)]
                fn baz(_: baz::Context) {}
            };
        ),
        Settings {
            parse_cores: true,
            parse_schedule: true,
            ..Settings::default()
        },
    )
    .unwrap();

    assert_eq!(analysis.timer_queues[&0].priority, 3);
}

#[test]
fn shared() {
    let (app, analysis) = crate::parse2(
        quote!(cores = 2),
        quote!(
            const APP: () = {
                struct Resources {
                    #[init(0)]
                    #[shared]
                    x: u32,
                }

                #[init(core = 0)]
                fn init(cx: init::Context) {
                    #[shared]
                    static mut Y: [u8; 128] = [0; 128];
                }
            };
        ),
        Settings {
            parse_cores: true,
            ..Settings::default()
        },
    )
    .unwrap();

    let (name, loc) = analysis.locations.get_index(0).unwrap();
    assert_eq!(name.to_string(), "x");
    assert_eq!(
        *loc,
        Location::Shared {
            cores: BTreeSet::new()
        }
    );

    let (name, local) = app.inits[&0].locals.get_index(0).unwrap();
    assert_eq!(name.to_string(), "Y");
    assert!(local.shared);
}
