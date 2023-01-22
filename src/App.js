import React, { useContext, Fragment, useState } from "react";
import { Grid } from "./components/Grid";
import { Start } from "./components/Start";
import { SimulationContext } from "./contexts/Simulation";

const App = (props) => {
  const { name } = props;
  const { height, width } = useContext(SimulationContext);
  return (
    <Fragment>
      <h1 className="bg-black text-4xl text-white">Hello {name}!</h1>
      <Grid
        width={height}
        height={width}
      />
      <Start></Start>
    </Fragment>
  );
};

export default App;
