use leptos::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
  // Reactive signals - these are like react's useState
  // RwSignal means "readable and writable signal"
  let (output, set_output) = signal(Vec::<(String, String)>::new());
  let (input_value, set_input_value) = signal(String::new());
  let cwd = RwSignal::new("~".to_string());

  // This runs when the user presses Enter
  let on_submit = move |ev: web_sys::KeyboardEvent| {
    if ev.key() == "Enter" {
      let cmd = input_value.get();
      let trimmed = cmd.trim().to_string();

      if !trimmed.is_empty() {
        // For now, just echo the command back
        let response = format!("You typed: {}", trimmed);

        // update the output signal by pushing a new entry
        set_output.update(|out| {
          out.push((trimmed, response));
        });
      }

      // Clear the input
      set_input_value.set(String::new());
    }
  };

  view! {
    <div class="terminal-wrapper">
      <div class="terminal">
        <div class="terminal-output">
          {move || output.get().into_iter().enumerate().map(|(_i, (cmd, resp))| {
            view! {
              <div>
                <div class="output-command">
                  <span class="prompt-user">"visitor"</span>
                  <span class="prompt-at">"@"</span>
                  <span class="prompt-host">"portfolio"</span>
                  <span class="prompt-colon">":"</span>
                  <span class="prompt-path">"~"</span>
                  <span class="prompt-dollar">"$ "</span>
                  {cmd}
                </div>
                <div class="output-line">{resp}</div>
              </div>
            }
          }).collect::<Vec<_>>()}
        </div>
        <div class="input-line">
          <span class="prompt-user">"visitor"</span>
          <span class="prompt-at">"@"</span>
          <span class="prompt-host">"portfolio"</span>
          <span class="prompt-colon">":"</span>
          <span class="prompt-path">{move || cwd.get()}</span>
          <span class="prompt-dollar">"$ "</span>
          <input
            type="text"
            class="terminal-input"
            prop:value=move || input_value.get()
            on:input=move |ev| {
                set_input_value.set(event_target_value(&ev));
            }
            on:keydown=on_submit
            autofocus=true
            autocomplete="off"
            spellcheck="false"
          />
        </div>
      </div>
    </div>
  }
}
