# zequals
Use a computer for estimations, not calculations.

Inspired by a [Numberphile video](https://www.youtube.com/watch?v=aOJOfh2_4PE), this library provides the `zequals` operation and a data type to go along with that. The operation is basically ruthless rounding to one significant place which is totally sufficient in many cases; with modern computers we can get a very high precision, but that doesn’t really help if accuracy is so low that basically none of the digits can be trusted. Zequals embraces the philosophy of *good enough*.

## License
Copyright 2014-2022 The Rust Project Developers

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option. Files in the project may not be copied, modified, or distributed except according to those terms.
