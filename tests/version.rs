// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026  Nicolas Gabriel Cotti

use libsigrok_sys::{sigrok, sigrokdecode};

#[test]
fn sigrok_lib_version() {
    let current = unsafe { sigrok::sr_lib_version_current_get() };
    let revision = unsafe { sigrok::sr_lib_version_revision_get() };
    let age = unsafe { sigrok::sr_lib_version_age_get() };

    dbg!(current);
    dbg!(revision);
    dbg!(age);
}

#[test]
fn sigrokdecode_lib_version() {
    let current = unsafe { sigrokdecode::srd_lib_version_current_get() };
    let revision = unsafe { sigrokdecode::srd_lib_version_revision_get() };
    let age = unsafe { sigrokdecode::srd_lib_version_age_get() };

    dbg!(current);
    dbg!(revision);
    dbg!(age);
}
