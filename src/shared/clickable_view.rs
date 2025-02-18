use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;

    ClickableView = {{ClickableView}} {
        width: Fit, height: Fit
        show_bg: true
        draw_bg: {
            color: #fff
        }
    }
}

#[derive(Live)]
pub struct ClickableView {
    #[deref]
    view:View,
}

impl LiveHook for ClickableView {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ClickableView);
    }
}

#[derive(Clone, WidgetAction)]
pub enum ClickableViewAction {
    None,
    Click,
}

impl Widget for ClickableView {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn walk(&self) -> Walk {
        self.view.walk()
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl ClickableView {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ClickableViewAction),
    ) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    dispatch_action(cx, ClickableViewAction::Click);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                //self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                //self.animator_play(cx, id!(hover.off));
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct ClickableViewRef(WidgetRef);

impl ClickableViewRef {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let ClickableViewAction::Click = item.action() {
                return true;
            }
        }
        false
    }
}
