# \DeployKeyApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deploy_key**](DeployKeyApi.md#create_deploy_key) | **POST** /deploy_keys | 
[**delete_deploy_key**](DeployKeyApi.md#delete_deploy_key) | **DELETE** /deploy_keys/{key_id} | 
[**get_deploy_key**](DeployKeyApi.md#get_deploy_key) | **GET** /deploy_keys/{key_id} | 
[**list_deploy_keys**](DeployKeyApi.md#list_deploy_keys) | **GET** /deploy_keys | 



## create_deploy_key

> crate::models::DeployKey create_deploy_key()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeployKey**](deployKey.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deploy_key

> delete_deploy_key(key_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deploy_key

> crate::models::DeployKey get_deploy_key(key_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::DeployKey**](deployKey.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deploy_keys

> Vec<crate::models::DeployKey> list_deploy_keys()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DeployKey>**](deployKey.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

