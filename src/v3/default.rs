use std::{fmt, marker::PhantomData, task::Context, task::Poll};

use ntex::service::{Service, ServiceFactory};
use ntex::util::Ready;

use super::control::{ControlMessage, ControlResult, ControlResultKind};
use super::publish::Publish;
use super::Session;

/// Default publish service
pub struct DefaultPublishService<St, Err> {
    _t: PhantomData<(St, Err)>,
}

impl<St, Err> Default for DefaultPublishService<St, Err> {
    fn default() -> Self {
        Self { _t: PhantomData }
    }
}

impl<St, Err> ServiceFactory<Publish, Session<St>> for DefaultPublishService<St, Err> {
    type Response = ();
    type Error = Err;
    type Service = DefaultPublishService<St, Err>;
    type InitError = Err;
    type Future = Ready<Self::Service, Self::InitError>;

    fn new_service(&self, _: Session<St>) -> Self::Future {
        Ready::Ok(DefaultPublishService { _t: PhantomData })
    }
}

impl<St, Err> Service<Publish> for DefaultPublishService<St, Err> {
    type Response = ();
    type Error = Err;
    type Future = Ready<Self::Response, Self::Error>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, _: Publish) -> Self::Future {
        log::warn!("Publish service is disabled");
        Ready::Ok(())
    }
}

/// Default control service
pub struct DefaultControlService<S, E>(PhantomData<(S, E)>);

impl<S, E> Default for DefaultControlService<S, E> {
    fn default() -> Self {
        DefaultControlService(PhantomData)
    }
}

impl<S, E: fmt::Debug> ServiceFactory<ControlMessage<E>, Session<S>>
    for DefaultControlService<S, E>
{
    type Response = ControlResult;
    type Error = E;
    type InitError = E;
    type Service = DefaultControlService<S, E>;
    type Future = Ready<Self::Service, Self::InitError>;

    fn new_service(&self, _: Session<S>) -> Self::Future {
        Ready::Ok(DefaultControlService(PhantomData))
    }
}

impl<S, E: fmt::Debug> Service<ControlMessage<E>> for DefaultControlService<S, E> {
    type Response = ControlResult;
    type Error = E;
    type Future = Ready<Self::Response, Self::Error>;

    #[inline]
    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&self, pkt: ControlMessage<E>) -> Self::Future {
        log::warn!("MQTT3 Subscribe is not supported");

        Ready::Ok(match pkt {
            ControlMessage::Ping(ping) => ping.ack(),
            ControlMessage::Disconnect(disc) => disc.ack(),
            ControlMessage::Closed(msg) => msg.ack(),
            _ => {
                log::warn!("MQTT3 Control service is not configured, pkt: {:?}", pkt);
                ControlResult { result: ControlResultKind::Disconnect }
            }
        })
    }
}
