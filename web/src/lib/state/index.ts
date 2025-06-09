import { me } from "../api/auth";
import { useAuthStore } from "./auth";

export function init() {
  const { updateUser } = useAuthStore.getState();
  me().then(user => {
    updateUser(user);
  });
}
