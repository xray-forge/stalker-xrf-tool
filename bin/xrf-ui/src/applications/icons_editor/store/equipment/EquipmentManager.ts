import { clamp } from "@mui/x-data-grid/internals";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { IEquipmentResponse, IEquipmentSectionDescriptor } from "@/lib/icons";
import { blobToImage } from "@/lib/image";
import { EIconsEditorCommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface IEquipmentPngDescriptor {
  descriptors: Array<IEquipmentSectionDescriptor>;
  path: string;
  name: string;
  blob: Blob;
  image: HTMLImageElement;
}

export interface IEquipmentContext {
  equipmentActions: {
    open(spritePath: string, systemLtxPath: string): Promise<void>;
    close(): Promise<void>;
    setGridVisibility(isVisible: boolean): void;
    setGridSize(size: number): void;
  };
  isReady: boolean;
  isGridVisible: boolean;
  gridSize: number;
  spriteImage: Loadable<Optional<IEquipmentPngDescriptor>>;
}

export class EquipmentManager extends ContextManager<IEquipmentContext> {
  public context: IEquipmentContext = {
    equipmentActions: createActions({
      open: (spritePath: string, systemLtxPath: string) => this.openEquipmentProject(spritePath, systemLtxPath),
      close: () => this.closeEquipmentProject(),
      setGridVisibility: (isVisible: boolean) => this.setContext({ isGridVisible: isVisible }),
      setGridSize: (size: number) => this.setContext({ gridSize: Math.round(clamp(size, 10, 100)) }),
    }),
    gridSize: 50,
    isReady: false,
    isGridVisible: true,
    spriteImage: createLoadable(null),
  };

  public log: Logger = new Logger("equipment");

  public async onProvisionStarted(): Promise<void> {
    const response: IEquipmentResponse = await invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE);

    if (response) {
      this.log.info("Existing equipment sprite detected");

      this.setContext({
        isReady: true,
        spriteImage: createLoadable(await this.spriteFromResponse(response)),
      });
    } else {
      this.log.info("No existing sprite detected file");
      this.setContext({ isReady: true });
    }
  }

  public async openEquipmentProject(equipmentDdsPath: string, systemLtxPath: string): Promise<void> {
    this.log.info("Opening equipment project:", equipmentDdsPath, systemLtxPath);

    try {
      this.setContext({ spriteImage: createLoadable(null, true) });

      const response: IEquipmentResponse = await invoke(EIconsEditorCommand.OPEN_EQUIPMENT_SPRITE, {
        equipmentDdsPath,
        systemLtxPath,
      });

      this.log.info("Equipment project opened:", response);

      this.setContext({
        spriteImage: createLoadable(await this.spriteFromResponse(response)),
      });
    } catch (error) {
      this.log.error("Failed to open equipment project:", error);
      this.setContext({ spriteImage: createLoadable(null, false, error as Error) });
    }
  }

  public async closeEquipmentProject(): Promise<void> {
    this.log.info("Closing equipment");

    try {
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asLoading() }));

      await invoke(EIconsEditorCommand.CLOSE_EQUIPMENT_SPRITE);

      this.log.info("Equipment project closed");

      this.setContext({ spriteImage: createLoadable(null) });
    } catch (error) {
      this.log.error("Failed to close equipment project:", error);
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asFailed(new Error(error as string)) }));
    }
  }

  public async spriteFromResponse(response: IEquipmentResponse): Promise<IEquipmentPngDescriptor> {
    const blob: Blob = await fetch(convertFileSrc(response.name, "stream")).then((response) => response.blob());

    return {
      blob,
      descriptors: response.equipmentDescriptors,
      image: await blobToImage(blob),
      name: response.name,
      path: response.path,
    };
  }
}
