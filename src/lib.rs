
pub struct ResourceFork {
    header: FileHeader, // 0 of (16 bytes)         = 16
    system_data: Vec<u8>, // from 16 bit of (112 bytes)    = 128
    application_data: Vec<u8>, // from 128  of (128 bytes) = 246
}


pub struct FileHeader {
    data_block_offset: u32, // 0 of (4 bytes) = 4
    resource_map_offset: u32, // 4 of (4 bytes) = 8 
    len_data_block: u32, // 8 of (4 bytes) = 12
    len_resource_map: u32, // 12 of (4 bytes) = 16
}

pub fn parse(bytes: Vec<u8>) -> ResourceFork {
    let header = parse_header(&bytes);
    let system_data = parse_system_data(&bytes);
    let application_data = parse_application_data(&bytes);
    ResourceFork {
        header,
        system_data,
        application_data,
    }
}

pub fn pack(resource_fork: ResourceFork) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend(pack_header(resource_fork.header));
    bytes.extend(resource_fork.system_data);
    bytes.extend(resource_fork.application_data);
    bytes
}

fn parse_header(bytes: &Vec<u8>) -> FileHeader {
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

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
