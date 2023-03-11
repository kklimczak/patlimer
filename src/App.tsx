import './App.scss';
import {createSignal, Match, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {Races} from "./components/Races";
import {Pilot} from "./models";
import {initialState, StateProvider} from "./store";
import {Events} from "./components/Events";

function App() {
    invoke('init').then(console.log);

    const [tab, setTab] = createSignal("list" as "list" | "single");

  return (
      <StateProvider initialState={initialState}>
          <div class="container">
              <header class="header">PatLimer</header>
              <main class="main">
                  <Switch>
                      <Match when={tab() === "list"}>
                        <Events />
                      </Match>
                      <Match when={tab() === "single"}>
                          <Races />
                      </Match>
                  </Switch>
              </main>
          </div>
      </StateProvider>
  );
}

export default App;
