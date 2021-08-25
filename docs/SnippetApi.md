# \SnippetApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site_snippet**](SnippetApi.md#create_site_snippet) | **POST** /sites/{site_id}/snippets | 
[**delete_site_snippet**](SnippetApi.md#delete_site_snippet) | **DELETE** /sites/{site_id}/snippets/{snippet_id} | 
[**get_site_snippet**](SnippetApi.md#get_site_snippet) | **GET** /sites/{site_id}/snippets/{snippet_id} | 
[**list_site_snippets**](SnippetApi.md#list_site_snippets) | **GET** /sites/{site_id}/snippets | 
[**update_site_snippet**](SnippetApi.md#update_site_snippet) | **PUT** /sites/{site_id}/snippets/{snippet_id} | 



## create_site_snippet

> crate::models::Snippet create_site_snippet(site_id, snippet)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**snippet** | [**Snippet**](Snippet.md) |  | [required] |

### Return type

[**crate::models::Snippet**](snippet.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site_snippet

> delete_site_snippet(site_id, snippet_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**snippet_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_snippet

> crate::models::Snippet get_site_snippet(site_id, snippet_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**snippet_id** | **String** |  | [required] |

### Return type

[**crate::models::Snippet**](snippet.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_snippets

> Vec<crate::models::Snippet> list_site_snippets(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Snippet>**](snippet.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site_snippet

> update_site_snippet(site_id, snippet_id, snippet)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**snippet_id** | **String** |  | [required] |
**snippet** | [**Snippet**](Snippet.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

