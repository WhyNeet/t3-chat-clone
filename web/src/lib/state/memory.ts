import { create } from "zustand";
import type { Memory } from "../model/memory";

export interface MemoryStore {
  memories: Memory[] | null;
  setMemories: (memories: Memory[]) => void;
  removeMemory: (id: string) => void;
  addMemory: (memory: Memory) => void;
}

export const useMemoryStore = create<MemoryStore>(set => ({
  memories: null,
  setMemories: (memories) => set({ memories }),
  addMemory: (memory) => set(state => ({ memories: [...(state.memories ?? []), memory] })),
  removeMemory: (id) => set(state => ({ memories: state.memories?.filter(memory => memory.id !== id) }))
}))