import { HexString } from '@gear-js/api';

interface ContractSails {
  programId: HexString,
  idl: string
}

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: 'wss://testnet.vara.network', // import.meta.env.VITE_NODE_ADDRESS,
  BACK: import.meta.env.VITE_BACKEND_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS as HexString,
};

export const ROUTES = {
  HOME: '/',
  EXAMPLES: '/examples',
  NOTFOUND: '*',
};

// To use the example code, enter the details of the account that will pay the vouchers, etc. (name and mnemonic)
export const sponsorName = "";
export const sponsorMnemonic = "";

export const CONTRACT_DATA: ContractSails = {
  programId: '0x77f69068c262418a25abf4638c767e8a8a5d11ede8a2fda6f66ea24c0cae60d3',
  idl: `
type IoLowFileState = struct {
  nombre: str,
  edad: u32,
  profesion: str,
  titulacion: str,
  ubicacion: str,
  certificaciones: vec str,
  identi: str,
  public_key: vec u8,
  email: str,
  nickname: str,
};

constructor {
  New : ();
};

service LowFileService {
  SetUserData : (nombre: str, edad: u32, profesion: str, titulacion: str, ubicacion: str, certificaciones: vec str, identi: str, public_key: vec u8, email: str, nickname: str) -> null;
  query GetUserData : () -> IoLowFileState;
};
  `
};