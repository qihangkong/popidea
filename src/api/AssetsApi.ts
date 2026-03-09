export class AssetsApi {
  static async getAssetFolders(projectId: string, parentId?: string) {
    const { getAssetFolders } = await import('./assets')
    return getAssetFolders(projectId, parentId)
  }

  static async createAssetFolder(projectId: string, name: string, parentId?: string) {
    const { createAssetFolder } = await import('./assets')
    return createAssetFolder(projectId, name, parentId)
  }

  static async deleteAssetFolder(id: string) {
    const { deleteAssetFolder } = await import('./assets')
    return deleteAssetFolder(id)
  }

  static async getAssets(projectId: string, folderId?: string, assetType?: string) {
    const { getAssets } = await import('./assets')
    return getAssets(projectId, folderId, assetType)
  }

  static async createAsset(data: any) {
    const { createAsset } = await import('./assets')
    return createAsset(data)
  }

  static async deleteAsset(id: string) {
    const { deleteAsset } = await import('./assets')
    return deleteAsset(id)
  }
}
