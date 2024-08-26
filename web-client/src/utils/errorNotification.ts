import { notifications } from "@mantine/notifications";

export const error = (message: string, title: string | null = null) => {
  notifications.show({
    title,
    message,
  });
};
