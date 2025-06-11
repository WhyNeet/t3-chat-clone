import { me } from "../api/auth";
import { chats } from "../api/chats";
import { subscribeToStream } from "../api/completions";
import { useAuthStore } from "./auth";
import { useChatsStore } from "./chats";

export function init() {
  const { updateUser } = useAuthStore.getState();

  const { initializeChat, finishFetching, updatePendingMessage } = useChatsStore.getState();
  useAuthStore.subscribe((store, prev) => {
    if (store.user && !prev.user) {
      chats({
        start: 0,
        take: Math.round((window.innerHeight * 1.5) / 50),
      }).then((chats) => {
        for (const chat of chats) {
          initializeChat(chat);
          const streamId = localStorage.getItem(`stream-${chat.id}`);
          if (streamId) {
            const message = localStorage.getItem(`streaming-message-${chat.id}`) ?? "";
            updatePendingMessage(chat.id, message);
            subscribeToStream(streamId, (delta) => {
              updatePendingMessage(chat.id, delta.content);
            }, () => {
              localStorage.removeItem(`stream-${chat.id}`);
              localStorage.removeItem(`streaming-message-${chat.id}`);
            });
          }
        }
        finishFetching();
      });
    }
  });

  me().then((user) => {
    updateUser(user);
  });
}
