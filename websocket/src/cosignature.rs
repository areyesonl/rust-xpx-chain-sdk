// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use crate::Handler;

pub struct HandlerCosignature {
    pub handler: Box<dyn Fn(sdk::multisig::CosignatureInfo) -> bool + Send>
}

impl Handler for HandlerCosignature {}
