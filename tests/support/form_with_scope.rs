//! One `form_with url:, scope:` contract for the interpreted and native
//! lanes (`emit_and_run/form_with_scope.rs`,
//! `spinel_toolchain/form_with_scope.rs`).
//!
//! A form with a scope and no model names its fields under the scope but
//! has no object: Rails reads no values from a record and asks none for
//! errors, even when the scope names a model with validations. A failed
//! create renders the form again, filled from `params` alone. Each case
//! below is what Rails 8.1.4 answered for this app.

pub const SCHEMA: &str = r#"ActiveRecord::Schema[8.1].define do
  create_table "rooms", force: :cascade do |t|
    t.string "name", null: false
    t.string "host_nickname"
    t.boolean "listed", default: false, null: false
  end
end
"#;

pub const MODEL: &str = r#"class Room < ApplicationRecord
  validates :name, presence: true
end
"#;

pub const CONTROLLER: &str = r#"class RoomsController < ApplicationController
  def new
  end

  def create
    attributes = params.expect(room: %i[name host_nickname listed])
    room = Room.new(name: attributes[:name], host_nickname: attributes[:host_nickname])
    if room.save
      redirect_to new_room_path, status: :see_other
    else
      @errors = room.errors.full_messages
      render :new, status: :unprocessable_entity
    end
  end
end
"#;

pub const VIEW: &str = r#"<% if @errors.present? %><ul><% @errors.each do |message| %><li><%= message %></li><% end %></ul><% end %>
<%= form_with url: rooms_path, scope: :room do |form| %>
<%= form.label :name, "Room" %>
<%= form.text_field :name, value: params.dig(:room, :name), required: true, maxlength: 60 %>
<%= form.text_field :host_nickname, value: params.dig(:room, :host_nickname) %>
<%= form.check_box :listed %>
<%= form.hidden_field :kind, value: "contest" %>
<%= form.submit "Create" %>
<% end %>
"#;

pub const ROUTES: &str = "Rails.application.routes.draw do\n  resources :rooms, only: %i[new create]\nend\n";

pub struct Case {
    pub method: &'static str,
    pub path: &'static str,
    /// The urlencoded form body of a POST.
    pub form: &'static str,
    pub status: u16,
    /// The `<li>` messages above the form.
    pub alerts: &'static [&'static str],
    /// Each text and hidden input's `value` attribute, None where Rails
    /// renders none.
    pub values: &'static [(&'static str, Option<&'static str>)],
}

const EMPTY: &[(&str, Option<&str>)] =
    &[("room[name]", None), ("room[host_nickname]", None), ("room[kind]", Some("contest"))];

pub const CASES: &[Case] = &[
    Case { method: "GET", path: "/rooms/new", form: "", status: 200, alerts: &[], values: EMPTY },
    Case {
        method: "GET",
        path: "/rooms/new?room%5Bname%5D=Kitchen",
        form: "",
        status: 200,
        alerts: &[],
        values: &[("room[name]", Some("Kitchen")), ("room[host_nickname]", None), ("room[kind]", Some("contest"))],
    },
    Case {
        method: "POST",
        path: "/rooms",
        form: "room%5Bname%5D=&room%5Bhost_nickname%5D=%3Cb%3E%26%22x",
        status: 422,
        alerts: &["Name can&#39;t be blank"],
        values: &[
            ("room[name]", Some("")),
            ("room[host_nickname]", Some("&lt;b&gt;&amp;&quot;x")),
            ("room[kind]", Some("contest")),
        ],
    },
    Case {
        method: "POST",
        path: "/rooms",
        form: "room%5Bname%5D=Hall&room%5Bhost_nickname%5D=aki&room%5Blisted%5D=1",
        status: 303,
        alerts: &[],
        values: &[],
    },
];

pub fn overlay() -> super::emit_and_run::Overlay {
    super::emit_and_run::empty_app()
        .write("db/schema.rb", SCHEMA)
        .write(
            "app/models/application_record.rb",
            "class ApplicationRecord < ActiveRecord::Base\n  primary_abstract_class\nend\n",
        )
        .write("app/models/room.rb", MODEL)
        .write(
            "app/controllers/application_controller.rb",
            "class ApplicationController < ActionController::Base\n  skip_forgery_protection\nend\n",
        )
        .write("app/controllers/rooms_controller.rb", CONTROLLER)
        .write("app/views/rooms/new.html.erb", VIEW)
        .write("config/routes.rb", ROUTES)
}

/// The Ruby lane: each case through `Main.run_rack`, printed as a
/// `<status> <bytes>` line and the body, for [`responses`] to read back.
pub fn script() -> String {
    let mut script = String::from(
        "def call(method, path, form)\n  path, query = path.split(\"?\", 2)\n  env = { \"REQUEST_METHOD\" => method, \"PATH_INFO\" => path, \"QUERY_STRING\" => query.to_s,\n    \"HTTP_HOST\" => \"localhost\", \"rack.input\" => StringIO.new(form) }\n  env.merge!(\"CONTENT_TYPE\" => \"application/x-www-form-urlencoded\", \"CONTENT_LENGTH\" => form.bytesize.to_s) if method == \"POST\"\n  status, _, body = Main.run_rack(env)\n  body = body.join\n  $stdout.write(\"#{status} #{body.bytesize}\\n\", body)\nend\n",
    );
    for case in CASES {
        script.push_str(&format!("call({:?}, {:?}, {:?})\n", case.method, case.path, case.form));
    }
    script
}

/// `(status, body)` per case, from [`script`]'s output.
pub fn responses(mut out: &str) -> Vec<(u16, String)> {
    let mut all = Vec::new();
    while let Some((head, rest)) = out.split_once('\n') {
        let (status, len) = head.split_once(' ').unwrap_or_else(|| panic!("not a response header: {head:?}"));
        let len: usize = len.parse().expect("body length");
        all.push((status.parse().expect("status"), rest[..len].to_string()));
        out = &rest[len..];
    }
    all
}

/// Each tag carrying `name="<name>"`, in page order.
fn tags_named<'a>(body: &'a str, name: &str) -> Vec<&'a str> {
    let needle = format!("name=\"{name}\"");
    body.match_indices(&needle)
        .filter_map(|(at, _)| Some(&body[body[..at].rfind('<')?..at + body[at..].find('>')?]))
        .collect()
}

/// The `value` attribute of the first input named `name`, as written.
fn value_of<'a>(body: &'a str, name: &str) -> Option<Option<&'a str>> {
    let tag = *tags_named(body, name).first()?;
    Some(tag.split_once(" value=\"").map(|(_, rest)| &rest[..rest.find('"').unwrap_or(rest.len())]))
}

/// Every difference between one response and what Rails answered.
pub fn differences(case: &Case, status: u16, body: &str) -> Vec<String> {
    let mut diffs = Vec::new();
    let what = format!("{} {}", case.method, case.path);
    if status != case.status {
        diffs.push(format!("{what}: status {status}, want {}", case.status));
    }
    let alerts: Vec<&str> = body.split("<li>").skip(1).filter_map(|s| s.split_once("</li>")).map(|(m, _)| m).collect();
    if alerts != case.alerts {
        diffs.push(format!("{what}: alerts {alerts:?}, want {:?}", case.alerts));
    }
    for (name, want) in case.values {
        match value_of(body, name) {
            Some(got) if got == *want => {}
            Some(got) => diffs.push(format!("{what}: {name} value {got:?}, want {want:?}")),
            None => diffs.push(format!("{what}: no input named {name}")),
        }
    }
    if case.status != 303 {
        for absent in ["field_with_errors", "checked=\""] {
            if body.contains(absent) {
                diffs.push(format!("{what}: renders {absent}, which Rails does not"));
            }
        }
        if !tags_named(body, "room[listed]").iter().any(|tag| tag.contains("type=\"checkbox\"")) {
            diffs.push(format!("{what}: no room[listed] check box"));
        }
    }
    if !diffs.is_empty() {
        diffs.push(format!("{what} body:\n{body}"));
    }
    diffs
}
