import { reactive } from "vue";
import * as ffi from "@/lib/ffi";
import { ManagerStatus } from "@/lib/ffi";
import { Selection } from "./selection";
import { ListViewColumn } from "./view-columns";

export interface AppState {
  path: string | null;
  status: ManagerStatus | null;
  query: string;
  itemIds: number[];
  listViewColumns: ListViewColumn[];
  itemIdSelection: Selection | null;
}

// The app state. DO NOT MODIFY FROM CHILD COMPONENTS.
// You should only modify this using functions in this module.
export const state: AppState = reactive({
  path: null,
  status: null,
  query: "",
  itemIds: [],
  listViewColumns: [
    { type: "name", width: 300 },
    { type: "extension", width: 60 },
    { type: "path", width: 500 },
    { type: "tags", width: 200 },
  ],
  itemIdSelection: null,
});

const refreshFuncs: (() => Promise<void>)[] = [];

refreshFuncs.push(refreshStatus);
export async function refreshStatus() {
  const newState = await ffi.getStatus();
  if (newState !== state.status) {
    state.status = newState;
  }
}

refreshFuncs.push(refreshPath);
export async function refreshPath() {
  const newPath = await ffi.getRepoPath();
  if (newPath !== state.path) {
    state.path = newPath;
  }
}

export async function refreshAll() {
  for (const refreshFunc of refreshFuncs) {
    await refreshFunc();
  }
}

// fetch data right now
refreshAll().then()
