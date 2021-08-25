# \FileApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_site_file_by_path_name**](FileApi.md#get_site_file_by_path_name) | **GET** /sites/{site_id}/files/{file_path} | 
[**list_site_files**](FileApi.md#list_site_files) | **GET** /sites/{site_id}/files | 
[**upload_deploy_file**](FileApi.md#upload_deploy_file) | **PUT** /deploys/{deploy_id}/files/{path} | 



## get_site_file_by_path_name

> std::path::PathBuf get_site_file_by_path_name(site_id, file_path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**file_path** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_files

> Vec<std::path::PathBuf> list_site_files(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_deploy_file

> std::path::PathBuf upload_deploy_file(deploy_id, path, file_body, size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |
**path** | **String** |  | [required] |
**file_body** | **std::path::PathBuf** |  | [required] |
**size** | Option<**i32**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

