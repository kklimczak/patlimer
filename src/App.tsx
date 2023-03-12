import './App.scss';
import {createSignal, Match, Switch} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {Races} from "./components/Races";
import {Pilot} from "./models";
import {initialState, StateProvider, useAppState} from "./store";
import {Events} from "./components/Events";

function App() {
    invoke('init').then(console.log);

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
                          <Races />
                      </Match>
                  </Switch>
              </main>
          </div>
      </StateProvider>
  );
}

export default App;
