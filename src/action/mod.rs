// Copyright 2020 Google LLC
//
// Use of this source code is governed by an MIT-style license that can be found
// in the LICENSE file or at https://opensource.org/licenses/MIT.

pub mod startup;

pub trait Request {
    type Proto: prost::Message + Default;
    fn from_proto(proto: Self::Proto) -> Self;
}

pub trait Response {
    const RDF_NAME: Option<&'static str>;
    type Proto: prost::Message + Default;
    fn into_proto(self) -> Self::Proto;
}

impl Request for () {

    type Proto = ();

    fn from_proto(_: ()) {
    }
}

impl Response for () {

    const RDF_NAME: Option<&'static str> = None;

    type Proto = ();

    fn into_proto(self) {
    }
}
