use crate::data::alife::alife_actor::AlifeActor;
use crate::data::alife::alife_anomalous_zone::AlifeAnomalousZone;
use crate::data::alife::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::alife_level_changer::AlifeLevelChanger;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
use crate::data::alife::alife_object_helicopter::AlifeObjectHelicopter;
use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
use crate::data::alife::alife_object_item_artefact::AlifeObjectItemArtefact;
use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
use crate::data::alife::alife_object_item_detector::AlifeObjectItemDetector;
use crate::data::alife::alife_object_item_explosive::AlifeObjectItemExplosive;
use crate::data::alife::alife_object_item_grenade::AlifeObjectItemGrenade;
use crate::data::alife::alife_object_item_helmet::AlifeObjectItemHelmet;
use crate::data::alife::alife_object_item_pda::AlifeObjectItemPda;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use crate::data::alife::alife_object_item_weapon_magazined_wgl::AlifeObjectItemWeaponMagazinedWgl;
use crate::data::alife::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
use crate::data::alife::alife_object_physic::AlifeObjectPhysic;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife::alife_object_torrid_zone::AlifeObjectTorridZone;
use crate::data::alife::alife_smart_cover::AlifeSmartCover;
use crate::data::alife::alife_smart_terrain::AlifeSmartTerrain;
use crate::data::alife::alife_zone_visual::AlifeZoneVisual;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::DatabaseError;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use enum_map::Enum;
use xray_chunk::ChunkReader;
use xray_ltx::Ltx;

#[derive(Clone, Debug, Enum, PartialEq)]
pub enum AlifeClass {
  CseAlifeAnomalousZone,
  CseAlifeCar,
  CseAlifeCreatureCrow,
  CseAlifeDynamicObjectVisual,
  CseAlifeFleshGroup,
  CseAlifeGraphPoint,
  CseAlifeHelicopter,
  CseAlifeInventoryBox,
  CseAlifeItem,
  CseAlifeItemAmmo,
  CseAlifeItemArtefact,
  CseAlifeItemBolt,
  CseAlifeItemCustomOutfit,
  CseAlifeItemDetector,
  CseAlifeItemDocument,
  CseAlifeItemExplosive,
  CseAlifeItemGrenade,
  CseAlifeItemHelmet,
  CseAlifeItemPda,
  CseAlifeItemTorch,
  CseAlifeItemWeapon,
  CseAlifeItemWeaponMagazined,
  CseAlifeItemWeaponMagazinedWGl,
  CseAlifeItemWeaponShotgun,
  CseAlifeMonsterBase,
  CseAlifeMountedWeapon,
  CseAlifeObjectBreakable,
  CseAlifeObjectClimable,
  CseAlifeObjectHangingLamp,
  CseAlifeObjectPhysic,
  CseAlifeObjectProjector,
  CseAlifePhSkeletonObject,
  CseAlifeRatGroup,
  CseAlifeSpaceRestrictor,
  CseAlifeSpawnGroup,
  CseAlifeStationaryMGun,
  CseAlifeTeamBaseZone,
  CseAlifeTrader,
  CseSpectator,
  SeActor,
  SeLevelChanger,
  SeMonster,
  SeSimFaction,
  SeSmartCover,
  SeSmartTerrain,
  SeStalker,
  SeZoneAnom, // cse_anomalous_zone
  SeZoneTorrid,
  SeZoneVisual,
  SimSquadScripted,
  Unknown,
}

impl AlifeClass {
  /// Read custom save data based on serialized clsid.
  /// Represents STATE_Read of each separate object in xray implementation.
  /// Additionally, should respect script extension.
  pub fn read_by_class<T: ByteOrder>(
    reader: &mut ChunkReader,
    alife_class: &Self,
  ) -> DatabaseResult<Box<dyn AlifeObjectWriter>> {
    Ok(match alife_class {
      Self::SeActor => {
        let object: AlifeActor = AlifeActor::read::<T>(reader)?;
        AlifeActor::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeObjectBreakable => {
        let object: AlifeObjectBreakable = AlifeObjectBreakable::read::<T>(reader)?;
        AlifeObjectBreakable::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::read::<T>(reader)?;
        AlifeObjectClimable::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::read::<T>(reader)?;
        AlifeGraphPoint::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read::<T>(reader)?;
        AlifeObjectSpaceRestrictor::verify(reader);
        Box::new(object)
      }
      Self::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::read::<T>(reader)?;
        AlifeSmartCover::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeAnomalousZone => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::read::<T>(reader)?;
        AlifeObjectAnomalyZone::verify(reader);
        Box::new(object)
      }
      Self::SeZoneAnom => {
        let object: AlifeAnomalousZone = AlifeAnomalousZone::read::<T>(reader)?;
        AlifeAnomalousZone::verify(reader);
        Box::new(object)
      }
      Self::SeZoneTorrid => {
        let object: AlifeObjectTorridZone = AlifeObjectTorridZone::read::<T>(reader)?;
        AlifeObjectTorridZone::verify(reader);
        Box::new(object)
      }
      Self::SeSmartTerrain => {
        let object: AlifeSmartTerrain = AlifeSmartTerrain::read::<T>(reader)?;
        AlifeSmartTerrain::verify(reader);
        Box::new(object)
      }
      Self::SeLevelChanger => {
        let object: AlifeLevelChanger = AlifeLevelChanger::read::<T>(reader)?;
        AlifeLevelChanger::verify(reader);
        Box::new(object)
      }
      Self::SeZoneVisual => {
        let object: AlifeZoneVisual = AlifeZoneVisual::read::<T>(reader)?;
        AlifeZoneVisual::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeObjectPhysic => {
        let object: AlifeObjectPhysic = AlifeObjectPhysic::read::<T>(reader)?;
        AlifeObjectPhysic::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeHelicopter => {
        let object: AlifeObjectHelicopter = AlifeObjectHelicopter::read::<T>(reader)?;
        AlifeObjectHelicopter::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeInventoryBox => {
        let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox::read::<T>(reader)?;
        AlifeObjectInventoryBox::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeObjectHangingLamp => {
        let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp::read::<T>(reader)?;
        AlifeObjectHangingLamp::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItem => {
        let object: AlifeObjectItem = AlifeObjectItem::read::<T>(reader)?;
        AlifeObjectItem::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemExplosive => {
        let object: AlifeObjectItemExplosive = AlifeObjectItemExplosive::read::<T>(reader)?;
        AlifeObjectItemExplosive::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemPda => {
        let object: AlifeObjectItemPda = AlifeObjectItemPda::read::<T>(reader)?;
        AlifeObjectItemPda::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemAmmo => {
        let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo::read::<T>(reader)?;
        AlifeObjectItemAmmo::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemGrenade => {
        let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade::read::<T>(reader)?;
        AlifeObjectItemGrenade::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemArtefact => {
        let object: AlifeObjectItemArtefact = AlifeObjectItemArtefact::read::<T>(reader)?;
        AlifeObjectItemArtefact::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemWeapon => {
        let object: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read::<T>(reader)?;
        AlifeObjectItemWeapon::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemDetector => {
        let object: AlifeObjectItemDetector = AlifeObjectItemDetector::read::<T>(reader)?;
        AlifeObjectItemDetector::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemHelmet => {
        let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet::read::<T>(reader)?;
        AlifeObjectItemHelmet::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemCustomOutfit => {
        let object: AlifeObjectItemCustomOutfit = AlifeObjectItemCustomOutfit::read::<T>(reader)?;
        AlifeObjectItemCustomOutfit::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemWeaponShotgun => {
        let object: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun::read::<T>(reader)?;
        AlifeObjectItemWeaponShotgun::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemWeaponMagazined => {
        let object: AlifeObjectItemWeaponMagazined =
          AlifeObjectItemWeaponMagazined::read::<T>(reader)?;
        AlifeObjectItemWeaponMagazined::verify(reader);
        Box::new(object)
      }
      Self::CseAlifeItemWeaponMagazinedWGl => {
        let object: AlifeObjectItemWeaponMagazinedWgl =
          AlifeObjectItemWeaponMagazinedWgl::read::<T>(reader)?;
        AlifeObjectItemWeaponMagazinedWgl::verify(reader);
        Box::new(object)
      }
      _ => {
        return Err(DatabaseError::new_parse_error(format!(
          "Not implemented parser for {:?}",
          alife_class
        )));
      }
    })
  }

  /// Import custom save data based on serialized clsid.
  pub fn import_by_class(
    alife_class: &Self,
    section_name: &str,
    ltx: &Ltx,
  ) -> DatabaseResult<Box<dyn AlifeObjectWriter>> {
    Ok(match alife_class {
      Self::SeActor => Box::new(AlifeActor::import(section_name, ltx)?),
      Self::CseAlifeObjectBreakable => Box::new(AlifeObjectBreakable::import(section_name, ltx)?),
      Self::CseAlifeObjectClimable => Box::new(AlifeObjectClimable::import(section_name, ltx)?),
      Self::CseAlifeGraphPoint => Box::new(AlifeGraphPoint::import(section_name, ltx)?),
      Self::CseAlifeSpaceRestrictor => {
        Box::new(AlifeObjectSpaceRestrictor::import(section_name, ltx)?)
      }
      Self::SeSmartCover => Box::new(AlifeSmartCover::import(section_name, ltx)?),
      Self::CseAlifeAnomalousZone => Box::new(AlifeObjectAnomalyZone::import(section_name, ltx)?),
      Self::SeZoneAnom => Box::new(AlifeAnomalousZone::import(section_name, ltx)?),
      Self::SeZoneTorrid => Box::new(AlifeObjectTorridZone::import(section_name, ltx)?),
      Self::SeSmartTerrain => Box::new(AlifeSmartTerrain::import(section_name, ltx)?),
      Self::SeLevelChanger => Box::new(AlifeLevelChanger::import(section_name, ltx)?),
      Self::SeZoneVisual => Box::new(AlifeZoneVisual::import(section_name, ltx)?),
      Self::CseAlifeObjectPhysic => Box::new(AlifeObjectPhysic::import(section_name, ltx)?),
      Self::CseAlifeHelicopter => Box::new(AlifeObjectHelicopter::import(section_name, ltx)?),
      Self::CseAlifeInventoryBox => Box::new(AlifeObjectInventoryBox::import(section_name, ltx)?),
      Self::CseAlifeObjectHangingLamp => {
        Box::new(AlifeObjectHangingLamp::import(section_name, ltx)?)
      }
      Self::CseAlifeItem => Box::new(AlifeObjectItem::import(section_name, ltx)?),
      Self::CseAlifeItemExplosive => Box::new(AlifeObjectItemExplosive::import(section_name, ltx)?),
      Self::CseAlifeItemPda => Box::new(AlifeObjectItemPda::import(section_name, ltx)?),
      Self::CseAlifeItemAmmo => Box::new(AlifeObjectItemAmmo::import(section_name, ltx)?),
      Self::CseAlifeItemGrenade => Box::new(AlifeObjectItemGrenade::import(section_name, ltx)?),
      Self::CseAlifeItemArtefact => Box::new(AlifeObjectItemArtefact::import(section_name, ltx)?),
      Self::CseAlifeItemWeapon => Box::new(AlifeObjectItemWeapon::import(section_name, ltx)?),
      Self::CseAlifeItemDetector => Box::new(AlifeObjectItemDetector::import(section_name, ltx)?),
      Self::CseAlifeItemHelmet => Box::new(AlifeObjectItemHelmet::import(section_name, ltx)?),
      Self::CseAlifeItemCustomOutfit => {
        Box::new(AlifeObjectItemCustomOutfit::import(section_name, ltx)?)
      }
      Self::CseAlifeItemWeaponShotgun => {
        Box::new(AlifeObjectItemWeaponShotgun::import(section_name, ltx)?)
      }
      Self::CseAlifeItemWeaponMagazined => {
        Box::new(AlifeObjectItemWeaponMagazined::import(section_name, ltx)?)
      }
      Self::CseAlifeItemWeaponMagazinedWGl => Box::new(AlifeObjectItemWeaponMagazinedWgl::import(
        section_name,
        ltx,
      )?),
      _ => {
        return Err(DatabaseError::new_parse_error(format!(
          "Not implemented parser for {:?}",
          alife_class
        )));
      }
    })
  }
}
