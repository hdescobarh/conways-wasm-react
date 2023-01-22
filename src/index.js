import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import SimulationContextProvider from "./contexts/Simulation";

const root = ReactDOM.createRoot(document.getElementById("app"));
root.render(
  <React.StrictMode>
    <SimulationContextProvider>
      <App name="Hansu" />
    </SimulationContextProvider>
  </React.StrictMode>
);
