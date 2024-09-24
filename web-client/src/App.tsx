import {
  BrowserRouter as Router,
  Routes,
  Route,
  useLocation,
} from "react-router-dom";
import { pages } from "./constants";
import { Box, Button, Typography } from "@mui/material";
import { EditPage } from "./pages/EditPage";

const NavBar = () => {
  const location = useLocation();

  const page = pages.find((page) => page.path === location.pathname);

  return (
    <Box
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: 50,
        borderBottom: "1px solid #000",
      }}
    >
      <Button
        onClick={() => {
          history.back();
          window.dispatchEvent(new PopStateEvent("popstate"));
        }}
        style={{
          position: "absolute",
          left: 10,
        }}
      >
        Back
      </Button>
      <Typography>{page?.name}</Typography>
    </Box>
  );
};

export const App = () => {
  return (
    <Router>
      <Box
        style={{
          display: "flex",
          flexDirection: "column",
          width: "100dvw",
          height: "100dvh",
        }}
      >
        <NavBar />
        <Box
          style={{
            flex: 1,
            width: "100%",
            minHeight: 0,
          }}
        >
          <Routes>
            {pages.map((page) => (
              <Route
                key={page.path}
                path={page.path}
                element={page.component}
              />
            ))}
            <Route
              //route for editing files, /edit/:path (:path can have slashes)
              path="/edit/*"
              element={<EditPage />}
            />
          </Routes>
        </Box>
      </Box>
    </Router>
  );
};
