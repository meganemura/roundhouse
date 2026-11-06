//! One `skip_before_action` contract for the interpreted and native lanes
//! (`emit_and_run/skip_before_action.rs`,
//! `spinel_toolchain/skip_before_action.rs`).
//!
//! A controller's own `before_action` on one of its private methods is
//! spliced into the bodies of the actions it applies to; the rest of the
//! chain runs in the dispatcher's preamble. Either way the filter must
//! run exactly where Rails' callback chain runs it once every later
//! `skip_before_action` has narrowed it, and only once. Each `load_label`
//! sets `@label`, each action renders it, and each body below is what
//! Rails 8.1.4 answered.

/// `(name, superclass, declarations, own actions)`. Every controller
/// whose parent is `Application` defines `load_label`.
const CONTROLLERS: &[(&str, &str, &str, &[&str])] = &[
    ("Only", "Application", "before_action :load_label\nskip_before_action :load_label, only: [:index]", &["index", "show"]),
    ("Except", "Application", "before_action :load_label\nskip_before_action :load_label, except: [:show]", &["index", "show", "edit"]),
    ("Unscoped", "Application", "before_action :load_label\nskip_before_action :load_label", &["index", "show"]),
    ("Narrowed", "Application", "before_action :load_label, only: [:show, :edit]\nskip_before_action :load_label, only: [:show]", &["index", "show", "edit"]),
    ("Guarded", "Application", "before_action :load_label\nskip_before_action :load_label, if: :skipping?", &["index"]),
    ("Mixed", "Application", "before_action :load_label\nskip_before_action :load_label, only: [:index], if: -> { params[:skip] == \"1\" }", &["index", "show"]),
    ("Parent", "Application", "before_action :load_label", &["index", "show"]),
    ("Overriding", "Parent", "skip_before_action :load_label, only: [:index]", &["index", "show"]),
    ("GuardedChild", "Parent", "skip_before_action :load_label, unless: -> { params[:keep] == \"1\" }", &[]),
    ("Redeclared", "Unscoped", "before_action :load_label", &["index"]),
];

/// `(path, body)`: `@label` is set only where the filter ran.
pub const CASES: &[(&str, &str)] = &[
    ("/only", "index:"),
    ("/only/show", "show:loaded"),
    ("/except", "index:"),
    ("/except/show", "show:loaded"),
    ("/except/edit", "edit:"),
    ("/unscoped", "index:"),
    ("/unscoped/show", "show:"),
    ("/narrowed", "index:"),
    ("/narrowed/show", "show:"),
    ("/narrowed/edit", "edit:loaded"),
    ("/guarded", "index:loaded"),
    ("/guarded?skip=1", "index:"),
    ("/mixed", "index:"),
    ("/mixed/show", "show:loaded"),
    ("/mixed/show?skip=1", "show:"),
    ("/parent", "index:loaded"),
    ("/overriding", "index:"),
    ("/overriding/show", "show:loaded"),
    ("/guarded_child/show", "show:"),
    ("/guarded_child/show?keep=1", "show:loaded"),
    ("/redeclared", "index:loaded"),
];

fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn source(name: &str, parent: &str, declarations: &str, actions: &[&str]) -> String {
    let mut src = format!("class {name}Controller < {parent}Controller\n");
    for line in declarations.lines() {
        src.push_str(&format!("  {line}\n"));
    }
    for action in actions {
        src.push_str(&format!("\n  def {action}\n    render plain: \"{action}:#{{@label}}\"\n  end\n"));
    }
    if parent == "Application" {
        src.push_str("\n  private\n\n  def skipping?\n    params[:skip] == \"1\"\n  end\n");
        src.push_str("\n  def load_label\n    @label = \"loaded\"\n  end\n");
    }
    src.push_str("end\n");
    src
}

pub fn overlay() -> super::emit_and_run::Overlay {
    let mut routes = String::from("Rails.application.routes.draw do\n");
    let mut app = super::emit_and_run::empty_app()
        .write("db/schema.rb", "ActiveRecord::Schema[8.1].define do\nend\n")
        .write(
            "app/controllers/application_controller.rb",
            "class ApplicationController < ActionController::Base\nend\n",
        );
    for (name, parent, declarations, actions) in CONTROLLERS {
        app = app.write(
            &format!("app/controllers/{}_controller.rb", snake(name)),
            &source(name, parent, declarations, actions),
        );
    }
    let mut paths: Vec<&str> = CASES.iter().map(|(p, _)| p.split('?').next().unwrap()).collect();
    paths.dedup();
    for path in paths {
        let (file, action) = path[1..].split_once('/').unwrap_or((&path[1..], "index"));
        routes.push_str(&format!("  get \"{path}\", to: \"{file}#{action}\"\n"));
    }
    routes.push_str("end\n");
    app.write("config/routes.rb", &routes)
}

/// The Ruby lane: the same cases through `Main.run_rack`.
pub fn assertions() -> String {
    let mut script = String::from(
        "def get(path)\n  path, query = path.split(\"?\", 2)\n  status, _, body = Main.run_rack(\"REQUEST_METHOD\" => \"GET\", \"PATH_INFO\" => path, \"QUERY_STRING\" => query.to_s,\n    \"HTTP_HOST\" => \"localhost\", \"rack.input\" => StringIO.new(\"\"))\n  [status, body.join]\nend\nfailures = []\n",
    );
    for (path, body) in CASES {
        script.push_str(&format!(
            "status, body = get({path:?})\nfailures << \"{path}: #{{status}} #{{body.inspect}}, want {}\" unless status == 200 && body == {body:?}\n",
            body.replace('"', "'"),
        ));
    }
    script.push_str("raise failures.join(\"\\n\") unless failures.empty?\nputs \"skip_before_action contract passed\"\n");
    script
}
