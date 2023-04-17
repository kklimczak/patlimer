import { createSignal, For, Match, Show, Switch } from "solid-js";
import { useAppState } from "../store";
import { Races } from "./Races";

export function Event() {
  const [state, {raceEvents, races}] = useAppState();

  const [openSettings, setOpenSettings] = createSignal(false);

  const selectedRaceEvent = () => state.raceEvents.find(raceEvent => raceEvent.id === state.selectedRaceEventId);

  const isEmpty = () => !state.pilots.length || !state.races.length;

  return <div>
    <button onClick={() => raceEvents.clearSelection()}>Back to races</button>
    <h2>{selectedRaceEvent()?.name}</h2>
    <Show when={!isEmpty() && !openSettings()}>
      <button onClick={() => setOpenSettings(true)}>Settings</button>
    </Show>
    <Show when={openSettings()}>
      <button onClick={() => setOpenSettings(false)}>Back</button>
    </Show>
    <Switch>
      <Match when={!openSettings()}>
        <Show when={!isEmpty()} fallback={<button onClick={() => setOpenSettings(true)}>Go to Settings</button>}>
          <ul>
            <For each={state.races}>
              {(item) => <li>{item.name} {item.status}</li>}
            </For>
          </ul>

          <button onClick={() => races.startRace()}>Start Race</button>
        </Show>
      </Match>
      <Match when={openSettings()}>
        <Races />
      </Match>
    </Switch>
  </div>
}
