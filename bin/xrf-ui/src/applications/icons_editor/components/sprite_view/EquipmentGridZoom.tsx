import { default as AddIcon } from "@mui/icons-material/AddCircle";
import { default as RemoveIcon } from "@mui/icons-material/RemoveCircle";
import { Grid, IconButton } from "@mui/material";
import { ReactElement } from "react";

interface IEquipmentGridZoomProps {
  zoom: number;
  onZoomUp: () => void;
  onZoomDown: () => void;
}

export function EquipmentGridZoom({ zoom, onZoomUp, onZoomDown }: IEquipmentGridZoomProps): ReactElement {
  return (
    <Grid display={"flex"} alignItems={"center"} position={"absolute"} right={4} bottom={4}>
      <IconButton aria-label={"delete"} size={"small"} color={"primary"} onClick={onZoomDown}>
        <RemoveIcon />
      </IconButton>

      <Grid marginLeft={0.5} marginRight={0.5}>
        {zoom.toFixed(2)}
      </Grid>

      <IconButton aria-label={"delete"} size={"small"} color={"primary"} onClick={onZoomUp}>
        <AddIcon />
      </IconButton>
    </Grid>
  );
}
