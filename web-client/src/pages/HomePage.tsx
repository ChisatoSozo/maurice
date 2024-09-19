import { Button, Box } from "@mui/material";
import { pages } from "../constants";

export const HomePage = () => {
  return (
    <Box
      style={{
        flexDirection: "column",
        width: "100%",
        display: "flex",
        height: "100%",
      }}
    >
      {pages.map((page) => (
        <Button
          style={{
            flex: 1,
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            width: "100%",
          }}
          onClick={() => {
            history.pushState({}, "", page.path);
            window.dispatchEvent(new PopStateEvent("popstate"));
          }}
        >
          {page.name}
        </Button>
      ))}
    </Box>
  );
};
