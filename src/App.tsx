import './App.scss';
import {Match, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {initialState, StateProvider, useAppState} from "./store";
import {Events} from "./components/Events";
import {Event} from "./components/Event";

function App() {
    const [state] = useAppState();

  return (
      <StateProvider initialState={initialState}>
          <div class="container">
              <header class="header">PatLimer</header>
              <main class="main">
                  <Switch>
                      <Match when={!state.selectedRaceEventId}>
                        <Events />
                      </Match>
                      <Match when={state.selectedRaceEventId}>
                          <Event />
                      </Match>
                  </Switch>
              </main>
          </div>
      </StateProvider>
  );
}

export default App;
