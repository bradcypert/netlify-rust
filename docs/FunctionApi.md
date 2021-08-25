# \FunctionApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_deploy_function**](FunctionApi.md#upload_deploy_function) | **PUT** /deploys/{deploy_id}/functions/{name} | 



## upload_deploy_function

> crate::models::Function upload_deploy_function(deploy_id, name, file_body, runtime, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**file_body** | **std::path::PathBuf** |  | [required] |
**runtime** | Option<**String**> |  |  |
**size** | Option<**i32**> |  |  |

### Return type

[**crate::models::Function**](function.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

