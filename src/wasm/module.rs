use super::sections;

#[derive(Debug)]
pub struct Module {
    version: [u8; 4],
    sections: Vec<Box<dyn sections::Section>>,
}

impl Module {
    const MAGIC_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
    const DEFAULT_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

    pub fn with(sections: Vec<Box<dyn sections::Section>>) -> Module {
        Module {
            version: Module::DEFAULT_VERSION,
            sections,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(&Module::MAGIC_HEADER);
        output.extend_from_slice(&self.version);

        for section in &self.sections {
            output.extend_from_slice(&section.encode());
        }

        output
    }
}
