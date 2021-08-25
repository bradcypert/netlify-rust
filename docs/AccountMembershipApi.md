# \AccountMembershipApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_account**](AccountMembershipApi.md#cancel_account) | **DELETE** /accounts/{account_id} | 
[**create_account**](AccountMembershipApi.md#create_account) | **POST** /accounts | 
[**get_account**](AccountMembershipApi.md#get_account) | **GET** /accounts/{account_id} | 
[**list_accounts_for_user**](AccountMembershipApi.md#list_accounts_for_user) | **GET** /accounts | 
[**update_account**](AccountMembershipApi.md#update_account) | **PUT** /accounts/{account_id} | 



## cancel_account

> cancel_account(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account

> crate::models::AccountMembership create_account(account_setup)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_setup** | [**AccountSetup**](AccountSetup.md) |  | [required] |

### Return type

[**crate::models::AccountMembership**](accountMembership.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> Vec<crate::models::AccountMembership> get_account(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::AccountMembership>**](accountMembership.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts_for_user

> Vec<crate::models::AccountMembership> list_accounts_for_user()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AccountMembership>**](accountMembership.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> crate::models::AccountMembership update_account(account_id, account_update_setup)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**account_update_setup** | Option<[**AccountUpdateSetup**](AccountUpdateSetup.md)> |  |  |

### Return type

[**crate::models::AccountMembership**](accountMembership.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

