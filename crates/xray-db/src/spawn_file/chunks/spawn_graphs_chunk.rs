use crate::data::graph::graph_cross_table::GraphCrossTable;
use crate::data::graph::graph_edge::GraphEdge;
use crate::data::graph::graph_header::GraphHeader;
use crate::data::graph::graph_level::GraphLevel;
use crate::data::graph::graph_level_point::GraphLevelPoint;
use crate::data::graph::graph_vertex::GraphVertex;
use crate::export::file::{create_export_file, open_binary_file, open_ltx_config};
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::Ltx;

/// `GameGraph::CHeader::load`, `GameGraph::SLevel::load`, `CGameGraph::Initialize`
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnGraphsChunk {
  pub header: GraphHeader,
  pub levels: Vec<GraphLevel>,
  pub vertices: Vec<GraphVertex>,
  pub edges: Vec<GraphEdge>,
  pub points: Vec<GraphLevelPoint>,
  pub cross_tables: Vec<GraphCrossTable>,
}

impl SpawnGraphsChunk {
  pub const CHUNK_ID: u32 = 4;

  /// Read graphs chunk by position descriptor.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading graphs chunk, bytes {:?}",
      reader.read_bytes_remain()
    );

    let mut levels: Vec<GraphLevel> = Vec::new();
    let mut vertices: Vec<GraphVertex> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut points: Vec<GraphLevelPoint> = Vec::new();

    let header: GraphHeader = GraphHeader::read::<T>(reader)?;

    for _ in 0..header.levels_count {
      levels.push(GraphLevel::read::<T>(reader)?)
    }

    for _ in 0..header.vertices_count {
      vertices.push(GraphVertex::read::<T>(reader)?);
    }

    for _ in 0..header.edges_count {
      edges.push(GraphEdge::read::<T>(reader)?);
    }

    for _ in 0..header.points_count {
      points.push(GraphLevelPoint::read::<T>(reader)?);
    }

    let cross_tables: Vec<GraphCrossTable> = GraphCrossTable::read_list::<T>(reader)?;

    log::info!(
      "Read graphs ver {:?}, {:?} bytes",
      header.version,
      reader.read_bytes_len(),
    );

    assert_eq!(levels.len(), header.levels_count as usize);
    assert_eq!(vertices.len(), header.vertices_count as usize);
    assert_eq!(edges.len(), header.edges_count as usize);
    assert_eq!(points.len(), header.points_count as usize);
    assert_eq!(cross_tables.len(), header.levels_count as usize);

    assert!(
      reader.is_ended(),
      "Expect graphs chunk to be ended, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self {
      header,
      levels,
      vertices,
      edges,
      points,
      cross_tables,
    })
  }

  /// Write whole graphs chunk into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.header.write::<T>(writer)?;

    for level in &self.levels {
      level.write::<T>(writer)?;
    }

    for vertex in &self.vertices {
      vertex.write::<T>(writer)?;
    }

    for edge in &self.edges {
      edge.write::<T>(writer)?;
    }

    for point in &self.points {
      point.write::<T>(writer)?;
    }

    GraphCrossTable::write_list::<T>(&self.cross_tables, writer)?;

    log::info!("Written graphs chunk, {:?} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import graphs data from provided path.
  pub fn import<T: ByteOrder>(path: &Path) -> DatabaseResult<Self> {
    let header: GraphHeader =
      GraphHeader::import("header", &open_ltx_config(&path.join("graphs_header.ltx"))?)?;

    let levels_ltx: Ltx = open_ltx_config(&path.join("graphs_levels.ltx"))?;
    let mut levels: Vec<GraphLevel> = Vec::new();

    for index in 0..header.levels_count {
      levels.push(GraphLevel::import(&index.to_string(), &levels_ltx)?);
    }

    let vertices_ltx: Ltx = open_ltx_config(&path.join("graphs_vertices.ltx"))?;
    let mut vertices: Vec<GraphVertex> = Vec::new();

    for index in 0..header.vertices_count {
      vertices.push(GraphVertex::import(&index.to_string(), &vertices_ltx)?);
    }

    let points_ltx: Ltx = open_ltx_config(&path.join("graphs_points.ltx"))?;
    let mut points: Vec<GraphLevelPoint> = Vec::new();

    for index in 0..header.points_count {
      points.push(GraphLevelPoint::import(&index.to_string(), &points_ltx)?);
    }

    let edges_ltx: Ltx = open_ltx_config(&path.join("graphs_edges.ltx"))?;
    let mut edges: Vec<GraphEdge> = Vec::new();

    for index in 0..header.edges_count {
      edges.push(GraphEdge::import(&index.to_string(), &edges_ltx)?);
    }

    let cross_tables: Vec<GraphCrossTable> = GraphCrossTable::import_list::<T>(
      &mut open_binary_file(&path.join("graphs_cross_tables.gct"))?,
    )?;

    log::info!("Imported graphs chunk");

    Ok(Self {
      header,
      levels,
      vertices,
      edges,
      points,
      cross_tables,
    })
  }

  /// Export graphs data into provided path.
  /// Constructs many files with contained data.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> DatabaseResult {
    let mut graphs_header_ltx: Ltx = Ltx::new();

    self.header.export(&mut graphs_header_ltx);

    graphs_header_ltx.write_to(&mut create_export_file(&path.join("graphs_header.ltx"))?)?;

    let mut graphs_level_ltx: Ltx = Ltx::new();

    for (index, level) in self.levels.iter().enumerate() {
      level.export(&index.to_string(), &mut graphs_level_ltx);
    }

    graphs_level_ltx.write_to(&mut create_export_file(&path.join("graphs_levels.ltx"))?)?;

    log::info!("Exported graph levels");

    let mut graphs_vertices_ltx: Ltx = Ltx::new();

    for (index, vertex) in self.vertices.iter().enumerate() {
      vertex.export(&index.to_string(), &mut graphs_vertices_ltx);
    }

    graphs_vertices_ltx.write_to(&mut create_export_file(&path.join("graphs_vertices.ltx"))?)?;

    log::info!("Exported graph vertices");

    let mut graphs_points_ltx: Ltx = Ltx::new();

    for (index, point) in self.points.iter().enumerate() {
      point.export(&index.to_string(), &mut graphs_points_ltx);
    }

    graphs_points_ltx.write_to(&mut create_export_file(&path.join("graphs_points.ltx"))?)?;

    log::info!("Exported graph points");

    let mut graphs_edges_ltx: Ltx = Ltx::new();

    for (index, edge) in self.edges.iter().enumerate() {
      edge.export(&index.to_string(), &mut graphs_edges_ltx);
    }

    graphs_edges_ltx.write_to(&mut create_export_file(&path.join("graphs_edges.ltx"))?)?;

    log::info!("Exported graph edges");

    GraphCrossTable::export_list::<T>(
      &self.cross_tables,
      &mut create_export_file(&path.join("graphs_cross_tables.gct"))?,
    )?;

    log::info!("Exported graph cross tables");

    log::info!("Exported graphs chunk");

    Ok(())
  }
}

impl fmt::Debug for SpawnGraphsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "GraphsChunk {{ header: {:?}, levels: Vector[{}], vertices: Vector[{}] }}",
      self.header,
      self.levels.len(),
      self.vertices.len(),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::graph::graph_cross_table::GraphCrossTable;
  use crate::data::graph::graph_edge::GraphEdge;
  use crate::data::graph::graph_header::GraphHeader;
  use crate::data::graph::graph_level::GraphLevel;
  use crate::data::graph::graph_level_point::GraphLevelPoint;
  use crate::data::graph::graph_vertex::GraphVertex;
  use crate::spawn_file::chunks::spawn_graphs_chunk::SpawnGraphsChunk;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use uuid::uuid;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_empty() -> DatabaseResult {
    let filename: String = String::from("read_write_empty.chunk");

    let original: SpawnGraphsChunk = SpawnGraphsChunk {
      header: GraphHeader {
        version: 10,
        vertices_count: 0,
        edges_count: 0,
        points_count: 0,
        guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
        levels_count: 0,
      },
      levels: vec![],
      vertices: vec![],
      edges: vec![],
      points: vec![],
      cross_tables: vec![],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      SpawnGraphsChunk::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = String::from("read_write.chunk");

    let original: SpawnGraphsChunk = SpawnGraphsChunk {
      header: GraphHeader {
        version: 12,
        vertices_count: 2,
        edges_count: 4,
        points_count: 3,
        guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
        levels_count: 2,
      },
      levels: vec![
        GraphLevel {
          id: 25,
          name: String::from("test-level-1"),
          section: String::from("test-level-section-1"),
          guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
          offset: Vector3d::new(2.5, 4.55, -6.5),
        },
        GraphLevel {
          id: 26,
          name: String::from("test-level"),
          section: String::from("test-level-section"),
          guid: uuid!("89e55023-10b1-426f-9247-bb680e5fe0c8"),
          offset: Vector3d::new(3.5, 5.55, -7.5),
        },
      ],
      vertices: vec![
        GraphVertex {
          level_point: Vector3d::new(12.5, 11.6, 12.3),
          game_point: Vector3d::new(1.5, -4.0, 1000.0),
          level_id: 255,
          level_vertex_id: 4000,
          vertex_type: (1, 2, 3, 4),
          edges_offset: 540,
          level_points_offset: 4000,
          edges_count: 252,
          level_points_count: 253,
        },
        GraphVertex {
          level_point: Vector3d::new(43.5, 15.6, 0.3),
          game_point: Vector3d::new(0.5, -4.0, 44.0),
          level_id: 255,
          level_vertex_id: 3000,
          vertex_type: (4, 2, 4, 4),
          edges_offset: 31,
          level_points_offset: 623,
          edges_count: 252,
          level_points_count: 23,
        },
      ],
      edges: vec![
        GraphEdge {
          game_vertex_id: 713,
          distance: 21.50,
        },
        GraphEdge {
          game_vertex_id: 714,
          distance: 8443.50,
        },
        GraphEdge {
          game_vertex_id: 715,
          distance: 4.50,
        },
        GraphEdge {
          game_vertex_id: 716,
          distance: 3.0,
        },
      ],
      points: vec![
        GraphLevelPoint {
          position: Vector3d::new(1.5, 11.6, 12.7),
          distance: 100.50,
          level_vertex_id: 8000,
        },
        GraphLevelPoint {
          position: Vector3d::new(2.5, 11.6, 12.7),
          distance: 200.50,
          level_vertex_id: 8001,
        },
        GraphLevelPoint {
          position: Vector3d::new(3.5, 11.6, 12.7),
          distance: 300.50,
          level_vertex_id: 8002,
        },
      ],
      cross_tables: vec![
        GraphCrossTable {
          version: 16,
          nodes_count: 51,
          vertices_count: 4000,
          level_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
          game_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b8"),
          data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        },
        GraphCrossTable {
          version: 16,
          nodes_count: 4232,
          vertices_count: 3000,
          level_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
          game_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b8"),
          data: vec![1, 2, 3, 4, 5],
        },
      ],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 430);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 430);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 430 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      SpawnGraphsChunk::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
