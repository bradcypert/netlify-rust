# \MemberApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_member_to_account**](MemberApi.md#add_member_to_account) | **POST** /{account_slug}/members | 
[**list_members_for_account**](MemberApi.md#list_members_for_account) | **GET** /{account_slug}/members | 



## add_member_to_account

> Vec<crate::models::Member> add_member_to_account(account_slug, email, role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** |  | [required] |
**email** | **String** |  | [required] |
**role** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Member>**](member.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_members_for_account

> Vec<crate::models::Member> list_members_for_account(account_slug)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Member>**](member.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

