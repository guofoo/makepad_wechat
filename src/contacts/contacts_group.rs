use crate::contacts::contact_info::*;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::image::*;

    IMG_DEFAULT_AVATAR = dep("crate://self/resources/img/default_avatar.png")
    IMG_FILE_TRANSFER_AVATAR = dep("crate://self/resources/img/file_transfer_avatar.png")
    IMG_WECHAT_AVATAR = dep("crate://self/resources/img/wechat_avatar.png")

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    Divider = <View> {
        width: Fill,
        height: Fit,
        flow: Down
        <RoundedView> {
            width: Fill,
            height: 1.,
            draw_bg: {color: (#ddd)}
        }
    }

    ContactItem = <View> {
        width: Fill,
        height: Fit,
        padding: {left: 10., top: 10., bottom: 4.}, flow: Down

        content = <View> {
            width: Fill,
            height: Fit,
            padding: {top: 4., bottom: 6.}, align: {x: 0.0, y: 0.5}, spacing: 10., flow: Right
            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                width: 36.,
                height: 36.
            }

            label = <Label> {
                width: Fit,
                height: Fit
                draw_text: {
                    color: #000,
                    text_style: <REGULAR_TEXT>{},
                }
            }
        }

        <Divider> {}
    }

    ContactsGroup = {{ContactsGroup}} {
        width: Fill,
        height: Fit,
        margin: {left: 6.0},
        padding: {top: 20.}, spacing: 0., flow: Down

        header: <View> {
            width: Fit,
            height: Fit,
            padding: {left: 10., top: 10., bottom: 0.}

            label = <Label> {
                width: Fit,
                height: Fit,
                draw_text: {
                    color: #777,
                    text_style: <REGULAR_TEXT>{font_size: 10.},
                }
            }
        }

        people_contact_template: <ContactItem> {}

        file_transfer_template: <ContactItem> {
            content = {
                avatar = {
                    source: (IMG_FILE_TRANSFER_AVATAR)
                }
            }
        }

        wechat_template: <ContactItem> {
            content = {
                avatar = {
                    source: (IMG_WECHAT_AVATAR)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ContactItemId(pub LiveId);

#[derive(Live)]
pub struct ContactsGroup {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    header:View,

    #[live]
    people_contact_template: Option<LivePtr>,
    #[live]
    file_transfer_template: Option<LivePtr>,
    #[live]
    wechat_template: Option<LivePtr>,

    #[rust]
    data: Vec<ContactInfo>,
    #[rust]
    contacts: ComponentMap<ContactItemId,ViewRef>,
}

impl LiveHook for ContactsGroup {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ContactsGroup);
    }
}

impl Widget for ContactsGroup {
    fn walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.header.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl ContactsGroup {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);
        let _ = self.header.draw_walk_widget(cx, walk);

        for contact in self.data.iter() {
            let contact_widget_id = LiveId::from_str(&contact.name).into();
            let current_contact = self.contacts.get_or_insert(cx, contact_widget_id, |cx| {
                let template = match contact.kind {
                    ContactKind::People => self.people_contact_template,
                    ContactKind::FileTransfer => self.file_transfer_template,
                    ContactKind::WeChat => self.wechat_template,
                };
               ViewRef::new_from_ptr(cx, template)
            });

            current_contact
                .label(id!(content.label))
                .set_text(&contact.name);
            let _ = current_contact.draw_walk_widget(cx, walk);
        }
        cx.end_turtle();
    }

    pub fn set_header_label(&mut self, text: &str) {
        let label = self.header.label(id!(label));
        label.set_text(text);
    }

    pub fn set_contacts(&mut self, data: Vec<ContactInfo>) {
        self.data = data;
    }
}
