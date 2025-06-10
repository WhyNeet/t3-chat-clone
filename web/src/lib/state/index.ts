import { me } from "../api/auth";
import { chats } from "../api/chats";
import { useAuthStore } from "./auth";
import { useChatsStore } from "./chats";

export function init() {
  const { updateUser } = useAuthStore.getState();

  const { initializeChat, finishFetching } = useChatsStore.getState();
  useAuthStore.subscribe((store, prev) => {
    if (store.user && !prev.user) {
      chats({
        start: 0,
        take: Math.round((window.innerHeight * 1.5) / 50),
      }).then((chats) => {
        for (const chat of chats) {
          initializeChat(chat);
        }
        finishFetching();
      });
    }
  });

  me().then((user) => {
    updateUser(user);
  });
}
