# \AssetApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site_asset**](AssetApi.md#create_site_asset) | **POST** /sites/{site_id}/assets | 
[**delete_site_asset**](AssetApi.md#delete_site_asset) | **DELETE** /sites/{site_id}/assets/{asset_id} | 
[**get_site_asset_info**](AssetApi.md#get_site_asset_info) | **GET** /sites/{site_id}/assets/{asset_id} | 
[**list_site_assets**](AssetApi.md#list_site_assets) | **GET** /sites/{site_id}/assets | 
[**update_site_asset**](AssetApi.md#update_site_asset) | **PUT** /sites/{site_id}/assets/{asset_id} | 



## create_site_asset

> crate::models::AssetSignature create_site_asset(site_id, name, size, content_type, visibility)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**size** | **i64** |  | [required] |
**content_type** | **String** |  | [required] |
**visibility** | Option<**String**> |  |  |

### Return type

[**crate::models::AssetSignature**](assetSignature.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site_asset

> delete_site_asset(site_id, asset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_asset_info

> crate::models::Asset get_site_asset_info(site_id, asset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |

### Return type

[**crate::models::Asset**](asset.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_assets

> Vec<crate::models::Asset> list_site_assets(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Asset>**](asset.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site_asset

> crate::models::Asset update_site_asset(site_id, asset_id, state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |
**state** | **String** |  | [required] |

### Return type

[**crate::models::Asset**](asset.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

