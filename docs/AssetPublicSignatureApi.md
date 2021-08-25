# \AssetPublicSignatureApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_site_asset_public_signature**](AssetPublicSignatureApi.md#get_site_asset_public_signature) | **GET** /sites/{site_id}/assets/{asset_id}/public_signature | 



## get_site_asset_public_signature

> crate::models::AssetPublicSignature get_site_asset_public_signature(site_id, asset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**asset_id** | **String** |  | [required] |

### Return type

[**crate::models::AssetPublicSignature**](assetPublicSignature.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

