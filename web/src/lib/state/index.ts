import { me } from "../api/auth";
import { chats } from "../api/chats";
import { subscribeToStream } from "../api/completions";
import { listModels } from "../api/service";
import { useAuthStore } from "./auth";
import { useChatsStore } from "./chats";
import { useServiceStore } from "./service";

export function init() {
  const { setModels } = useServiceStore.getState();

  listModels().then((models) => setModels(models));

  const { updateUser } = useAuthStore.getState();

  const { initializeChat, finishFetching, updatePendingMessage } =
    useChatsStore.getState();
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
            const message = localStorage.getItem(
              `streaming-message-${chat.id}`,
            ) ?? "";
            const reasoning = localStorage.getItem(
              `streaming-message-reasoning-${chat.id}`,
            ) ?? "";
            updatePendingMessage(chat.id, { content: message, reasoning });
            subscribeToStream(
              streamId,
              (delta) => {
                updatePendingMessage(chat.id, {
                  reasoning: delta.reasoning,
                  content: delta.reasoning ? null : delta.content,
                });
                localStorage.setItem(
                  `streaming-message-${chat.id}`,
                  (localStorage.getItem(`streaming-message-${chat.id}`) ?? "") +
                  delta.content,
                );
                localStorage.setItem(
                  `streaming-message-reasoning-${chat.id}`,
                  (localStorage.getItem(
                    `streaming-message-reasoning-${chat.id}`,
                  ) ?? "") + (delta.reasoning ?? ""),
                );
              },
              () => {
                localStorage.removeItem(`stream-${chat.id}`);
                localStorage.removeItem(`streaming-message-${chat.id}`);
                localStorage.removeItem(
                  `streaming-message-reasoning-${chat.id}`,
                );
              },
            );
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
