use gloo::timers::callback::Timeout;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SnackbarProps {
    #[prop_or(5000)]
    pub time: u32,
    pub message: String,
    #[prop_or(false)]
    pub display: bool,
}

pub struct Snackbar {
    display: bool,
    fade: bool,
}

impl Snackbar {
    fn set_display(&mut self, ctx: &Context<Self>) {
        let SnackbarProps { time, display, .. } = ctx.props().clone();

        if display {
            let link = ctx.link().clone();
            if time > 1000 {
                Timeout::new(time - 1000, move || link.send_message(())).forget();
            } else {
                Timeout::new(1000, move || link.send_message(())).forget();
                self.fade = true;
            }
        }

        self.display = display;
        self.fade = false;
    }
}

impl Component for Snackbar {
    type Message = ();

    type Properties = SnackbarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut snackbar = Self {
            display: true,
            fade: false,
        };
        snackbar.set_display(ctx);

        snackbar
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        if self.fade {
            self.display = false;
            self.fade = false;
        } else {
            self.fade = true;
            let link = ctx.link().clone();
            Timeout::new(1000, move || link.send_message(())).forget();
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.set_display(ctx);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { message, .. } = ctx.props();
        let mut classes = vec!["fixed, top-0, bg-gray-400", "text-white", "p-2", "rounded"];

        if self.fade {
            classes.push("animate-fade-out");
        } else {
            classes.push("animate-fade-in");
        }

        if !self.display {
            classes.push("invisible"); // Translates to display: none
        }

        html! {
            <div class={classes!(classes)}>{message}</div>
        }
    }
}
