
#[derive(Debug)]
pub struct ResourceFork {
    header: FileHeader,
    system_data: Vec<u8>,
    application_data: Vec<u8>,
}


#[derive(Debug)]
pub struct FileHeader {
    data_block_offset: u32,
    resource_map_offset: u32,
    len_data_block: u32,
    len_resource_map: u32,
}

impl FileHeader {
    fn from_bytes(bytes: &Vec<u8>) -> FileHeader {
        let data_block_offset = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let resource_map_offset = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let len_data_block = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let len_resource_map = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        FileHeader {
            data_block_offset,
            resource_map_offset,
            len_data_block,
            len_resource_map,
        }
    }

    fn from_vec(data_block_len: u32, resource_map_len: u32) -> FileHeader {
        FileHeader {
            0x10,
            resource_map_offset: data_block_offset + data_block_len,
            data_block_len,
            resource_map_len,
        }
    }

    fn get_resource_map_address(&self) -> Vec<u32> {
        vec![self.data_block_offset, self.data_block_offset + self.len_data_block]
    }

    fn get_data_block_address(&self) -> Vec<u32> {
        vec![self.resource_map_offset, self.resource_map_offset + self.len_resource_map]
    }
}

impl ResourceFork {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 16 {
            return None;
        }

        let header = FileHeader::from_bytes(&bytes[0..0x10]);
        if let Some(header) = header {
            let system_data_bytes = &bytes[0x10..0x80];
            let application_data_bytes = &bytes[0x80..];

            // unpack the system and application data


            Some(ResourceFork {
                header,
                system_data: system_data_bytes.to_vec(),
                application_data: application_data_bytes.to_vec(),
            })
        } else {
            None
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.header.to_bytes());
        bytes.extend(&self.system_data);
        bytes.extend(&self.application_data);
        bytes
    }
}


pub fn pack(resource_fork: ResourceFork) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(pack_header(resource_fork.header));
    bytes.extend(resource_fork.system_data);
    bytes.extend(resource_fork.application_data);
    bytes
}



fn pack_header(header: FileHeader) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(header.data_block_offset.to_be_bytes());
    bytes.extend(header.resource_map_offset.to_be_bytes());
    bytes.extend(header.len_data_block.to_be_bytes());
    bytes.extend(header.len_resource_map.to_be_bytes());
    bytes
}

fn parse_system_data(bytes: &Vec<u8>) -> Vec<u8> {
    let mut system_data = Vec::new();
    for i in 16..128 {
        system_data.push(bytes[i]);
    }
    system_data
}

fn parse_application_data(bytes: &Vec<u8>) -> Vec<u8> {
    let mut application_data = Vec::new();
    for i in 128..bytes.len() {
        application_data.push(bytes[i]);
    }
    application_data
}