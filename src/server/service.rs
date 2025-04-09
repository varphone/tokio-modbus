// SPDX-FileCopyrightText: Copyright (c) 2017-2025 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{future::Future, ops::Deref};

/// A Modbus server service.
pub trait Service {
    /// Requests handled by the service.
    ///
    /// Both [`tokio_modbus::Request`](crate::Request) and
    /// [`tokio_modbus::SlaveRequest`](crate::SlaveRequest)
    /// are possible choices.
    type Request;

    /// Responses sent by the service.
    ///
    /// Both [`tokio_modbus::Response`](crate::Response) and
    /// `Option<tokio_modbus::Response>` are possible choices.
    /// The latter allows to selectively ignore requests
    /// by not sending a response.
    type Response: Into<Option<crate::Response>>;

    /// Exceptional responses sent by the service.
    ///
    /// Use [`tokio_modbus::ExceptionCode`](crate::ExceptionCode) as default.
    type Exception: Into<crate::ExceptionCode>;

    /// The future response value.
    type Future<'a>: Future<Output = Result<Self::Response, Self::Exception>> + Send + 'a
    where
        Self: 'a;

    /// Process the request and return the response asynchronously.
    fn call(&self, req: Self::Request) -> Self::Future<'_>;
}

impl<D> Service for D
where
    D: Deref + ?Sized,
    D::Target: Service,
{
    type Request = <D::Target as Service>::Request;
    type Response = <D::Target as Service>::Response;
    type Exception = <D::Target as Service>::Exception;
    type Future<'a>
        = <D::Target as Service>::Future<'a>
    where
        Self: 'a;

    /// A forwarding blanket impl to support smart pointers around [`Service`].
    fn call(&self, req: Self::Request) -> Self::Future<'_> {
        self.deref().call(req)
    }
}
