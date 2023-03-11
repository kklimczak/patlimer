import './App.scss';
import {createSignal} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {Races} from "./components/Races";
import {Pilot} from "./models";
import {initialState, StateProvider} from "./store";

function App() {
    invoke('init').then(console.log);

  return (
      <StateProvider initialState={initialState}>
          <div class="container">
              <header class="header">PatLimer</header>
              <main class="main">
                  <Races />
              </main>
          </div>
      </StateProvider>
  );
}

export default App;
