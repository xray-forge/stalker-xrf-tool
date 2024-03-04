import { Button, ButtonGroup, Card, Grid } from "@mui/material";
import { ReactElement } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

interface ISpawnBackButtonProps {
  disabled?: boolean;
}

export function SpawnBackButton({ disabled }: ISpawnBackButtonProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  return (
    <Card sx={{ minWidth: 200, marginTop: 2 }}>
      <Grid direction={"column"} container>
        <ButtonGroup orientation={"vertical"}>
          <Button disabled={disabled} onClick={() => navigate("/spawn_editor", { replace: true })}>
            Back
          </Button>
        </ButtonGroup>
      </Grid>
    </Card>
  );
}