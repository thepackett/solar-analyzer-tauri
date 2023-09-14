use std::marker::PhantomData;

use crossbeam::channel::TrySendError;
use thiserror::Error;

#[derive(Clone)]
pub struct ComponentChannelRx<T>{
    rx: crossbeam::channel::Receiver<T>,
    channel_id: u32,
}

#[derive(Debug, Error)]
pub enum ComponentChannelRxError {
    #[error("The window does not exist.")]
    NoWindow,
    #[error("The document does not exist.")]
    NoDocument,
    #[error("The root does not exist.")]
    NoRoot,
}




impl<T> ComponentChannelRx<T> 
    // IN: 'static,
    // OUT: 'static,
    // F: yew::callback::Callback<IN, OUT>
{
    pub fn new(rx: crossbeam::channel::Receiver<T>, id: u32) -> Self {
        ComponentChannelRx::<T>{
            rx: rx,
            channel_id: id,
        }
    }

    pub fn receiver(&self) -> &crossbeam::channel::Receiver<T> {
        &self.rx
    }

    pub fn get_event_listener(&self, callback: yew::callback::Callback<web_sys::Event>) -> Result<gloo_events::EventListener, ComponentChannelRxError> 
    {
        let root = web_sys::window().ok_or(ComponentChannelRxError::NoWindow)?
        .document().ok_or(ComponentChannelRxError::NoDocument)?
        .document_element().ok_or(ComponentChannelRxError::NoRoot)?;
        let listener = gloo_events::EventListener::new(
            &root, 
            format!("rust_channel_{}", self.channel_id), 
            move |e| callback.emit(e.clone())
        );
        Ok(listener)
    }

    

}

impl<T> PartialEq for ComponentChannelRx<T> {
    fn eq(&self, other: &Self) -> bool {
        //Since this will be a component property, we need to implement PartialEq. Yew checks to see if the props for a component have updated using partial equality.
        //If the props have changed, then it will then run the changed function, and if that returns true (which it does by default), it will then rerun the view function.
        //Since we want to be able to handle message passing via channels, we need to hook into this PartialEq check. Thus, we define that two receivers are equal if
        //  they connect to the same channel, and both receivers have no messages to process.
        self.rx.same_channel(&other.rx) && self.rx.is_empty() && other.rx.is_empty()
    }
}

#[derive(Clone)]
pub struct ComponentChannelTx<T>{
    tx: crossbeam::channel::Sender<T>,
    //channel_id is a identifying number that is likely, but not guaranteed to be unique. Will be used to make custom events to update components attached to this channel.
    channel_id: u32,
}

#[derive(Error, Debug)]
pub enum ComponentChannelTxError {
    #[error("The message queue for the channel is full.")]
    TrySendErrorFull,
    #[error("The channel is disconnected.")]
    TrySendErrorDisconnected,
    #[error("The window does not exist.")]
    NoWindow,
    #[error("The document does not exist.")]
    NoDocument,
    #[error("The root does not exist.")]
    NoRoot,
    #[error("Javascript error.")]
    JsError {
        error: wasm_bindgen::JsValue,
    }
}

impl<T> From<TrySendError<T>> for ComponentChannelTxError {
    fn from(value: TrySendError<T>) -> Self {
        match value {
            TrySendError::Full(_) => ComponentChannelTxError::TrySendErrorFull,
            TrySendError::Disconnected(_) => ComponentChannelTxError::TrySendErrorDisconnected,
        }
    }
}

impl From<wasm_bindgen::JsValue> for ComponentChannelTxError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        ComponentChannelTxError::JsError { error: value }
    }
}

impl<T> ComponentChannelTx<T> {
    pub fn new(tx: crossbeam::channel::Sender<T>, id: u32) -> Self {
        ComponentChannelTx::<T>{
            tx: tx,
            channel_id: id,
        }
    }

    pub fn sender(&self) -> &crossbeam::channel::Sender<T> {
        &self.tx
    }

    // fn channel_id(&self) -> u32 {
    //     self.channel_id
    // }

    pub fn try_send(&self, message: T) -> Result<(), ComponentChannelTxError> {
        //Define an inner function to reduce the amount of monomorphised code required for this function.
        fn send_event(id: u32) -> Result<(), ComponentChannelTxError> {
            let event = web_sys::Event::new(format!("rust_channel_{}", id).as_str())?;
            let root = web_sys::window().ok_or(ComponentChannelTxError::NoWindow)?
                .document().ok_or(ComponentChannelTxError::NoDocument)?
                .document_element().ok_or(ComponentChannelTxError::NoRoot)?;
            root.dispatch_event(&event)?;
            Ok(())
        }

        send_event(self.channel_id)?;
        self.tx.try_send(message)?;
        Ok(())
    }
}

impl<T> PartialEq for ComponentChannelTx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tx.same_channel(&other.tx)
    }
}

pub struct ComponentChannel<T>{
    _marker: PhantomData<T>
}

impl<T> ComponentChannel<T> {
    pub fn get(tx_rx_tuple: (crossbeam::channel::Sender<T>, crossbeam::channel::Receiver<T>)) -> (ComponentChannelTx<T>, ComponentChannelRx<T>) {
        let id = rand::random();
        (ComponentChannelTx::new(tx_rx_tuple.0, id), ComponentChannelRx::new(tx_rx_tuple.1, id))
    }
}