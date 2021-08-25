# \SiteApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site**](SiteApi.md#create_site) | **POST** /sites | 
[**create_site_in_team**](SiteApi.md#create_site_in_team) | **POST** /{account_slug}/sites | 
[**delete_site**](SiteApi.md#delete_site) | **DELETE** /sites/{site_id} | 
[**get_site**](SiteApi.md#get_site) | **GET** /sites/{site_id} | 
[**list_sites**](SiteApi.md#list_sites) | **GET** /sites | 
[**list_sites_for_account**](SiteApi.md#list_sites_for_account) | **GET** /{account_slug}/sites | 
[**unlink_site_repo**](SiteApi.md#unlink_site_repo) | **PUT** /sites/{site_id}/unlink_repo | 
[**update_site**](SiteApi.md#update_site) | **PATCH** /sites/{site_id} | 



## create_site

> crate::models::Site create_site(site, configure_dns)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site** | [**SiteSetup**](SiteSetup.md) |  | [required] |
**configure_dns** | Option<**bool**> |  |  |

### Return type

[**crate::models::Site**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_site_in_team

> crate::models::Site create_site_in_team(account_slug, configure_dns, site)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** |  | [required] |
**configure_dns** | Option<**bool**> |  |  |
**site** | Option<[**SiteSetup**](SiteSetup.md)> |  |  |

### Return type

[**crate::models::Site**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site

> delete_site(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site

> crate::models::Site get_site(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**crate::models::Site**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sites

> Vec<crate::models::Site> list_sites(name, filter, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**filter** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Site>**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sites_for_account

> Vec<crate::models::Site> list_sites_for_account(account_slug, name, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** |  | [required] |
**name** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Site>**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_site_repo

> crate::models::Site unlink_site_repo(site_id)


[Beta] Unlinks the repo from the site.  This action will also: - Delete associated deploy keys - Delete outgoing webhooks for the repo - Delete the site's build hooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**crate::models::Site**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site

> crate::models::Site update_site(site_id, site)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**site** | [**SiteSetup**](SiteSetup.md) |  | [required] |

### Return type

[**crate::models::Site**](site.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

