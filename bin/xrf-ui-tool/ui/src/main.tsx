import { default as CssBaseline } from "@mui/material/CssBaseline";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { Application } from "@/Application";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import "@/main.css";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <CssBaseline />
    <Application />
  </StrictMode>
);
