use std::rc::Rc;
use yew::prelude::*;

pub struct SharedDataContext<T> 
        where
        T: PartialEq + 'static
{
    state: Rc<T>,
    replace_data_callback: Callback<Rc<T>>,
}

#[derive(PartialEq, Properties)]
pub struct SharedDataContextProps<T> 
        where
        T: PartialEq + 'static
{
    #[prop_or_default]
    pub children: Children,
    pub init: Rc<T>,
}

pub enum SharedDataContextMessage<T> {
    NewState(Rc<T>),
}

impl<T> Component for SharedDataContext<T> 
        where
        T: PartialEq + 'static
{
    type Message = SharedDataContextMessage<T>;
    type Properties = SharedDataContextProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let update_callback = ctx.link().callback(SharedDataContextMessage::NewState);

        SharedDataContext::<T> { 
            state: ctx.props().init.clone(),
            replace_data_callback: update_callback,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SharedDataContextMessage::NewState(state) => {
                self.state = state;
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html!(
                <ContextProvider<(Rc<T>,Callback<Rc<T>>)> context={(self.state.clone(),self.replace_data_callback.clone())}>
                        {for ctx.props().children.iter()}
                </ContextProvider<(Rc<T>,Callback<Rc<T>>)>>
        )
    }
}