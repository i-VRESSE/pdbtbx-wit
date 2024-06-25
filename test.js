import { pdbtbxApi } from './dist/pdbtbx_wit.js'

const pdb_code = '1A0A'
const response = await fetch(`https://files.rcsb.org/download/${pdb_code}.pdb`)
const content = await response.text()

const info = pdbtbxApi.pdb2pdbinfo(content)

console.log(info)
