# \HookApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hook_by_site_id**](HookApi.md#create_hook_by_site_id) | **POST** /hooks | 
[**delete_hook**](HookApi.md#delete_hook) | **DELETE** /hooks/{hook_id} | 
[**enable_hook**](HookApi.md#enable_hook) | **POST** /hooks/{hook_id}/enable | 
[**get_hook**](HookApi.md#get_hook) | **GET** /hooks/{hook_id} | 
[**list_hooks_by_site_id**](HookApi.md#list_hooks_by_site_id) | **GET** /hooks | 
[**update_hook**](HookApi.md#update_hook) | **PUT** /hooks/{hook_id} | 



## create_hook_by_site_id

> crate::models::Hook create_hook_by_site_id(site_id, hook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**hook** | [**Hook**](Hook.md) |  | [required] |

### Return type

[**crate::models::Hook**](hook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_hook

> delete_hook(hook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_hook

> crate::models::Hook enable_hook(hook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** |  | [required] |

### Return type

[**crate::models::Hook**](hook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hook

> crate::models::Hook get_hook(hook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** |  | [required] |

### Return type

[**crate::models::Hook**](hook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hooks_by_site_id

> Vec<crate::models::Hook> list_hooks_by_site_id(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Hook>**](hook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_hook

> crate::models::Hook update_hook(hook_id, hook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** |  | [required] |
**hook** | [**Hook**](Hook.md) |  | [required] |

### Return type

[**crate::models::Hook**](hook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

