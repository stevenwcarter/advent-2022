pub struct RadioStream {
    stream: Vec<char>,
}

impl RadioStream {
    pub fn new(stream: &str) -> Self {
        Self {
            stream: stream.chars().collect(),
        }
    }

    pub fn stream_packet_length() -> usize {
        4
    }
    pub fn message_packet_length() -> usize {
        14
    }

    pub fn get_stream_packet_index(self) -> u32 {
        Self::get_unique_packet_index(&self.stream, Self::stream_packet_length())
    }

    pub fn get_message_packet_index(self) -> u32 {
        Self::get_unique_packet_index(&self.stream, Self::message_packet_length())
    }

    fn get_unique_packet_index(stream: &[char], unique_length: usize) -> u32 {
        let mut index = 0;
        let stream_length = stream.len();
        while index < stream_length - unique_length {
            let mut stream_packet: Vec<char> = stream[index..index + unique_length].to_vec();
            assert_eq!(stream_packet.len(), unique_length);

            stream_packet.sort();
            stream_packet.dedup();
            if stream_packet.len() == unique_length {
                return index as u32 + unique_length as u32;
            }
            index += 1;
        }
        0
    }
}
