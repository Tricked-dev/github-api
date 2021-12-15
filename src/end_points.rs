use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub enum Methods {
    Get,
    Post,
    Patch,
    Delete,
    Put,
}

pub enum EndPoints {
    /// * tags meta
    /// * get `/`
    /// * docs https://docs.github.com/rest/overview/resources-in-the-rest-api#root-endpoint
    ///
    /// GitHub API Root
    /// Get Hypermedia links to resources accessible in GitHub's REST API
    Get(),
    /// * tags apps
    /// * get `/app`
    /// * docs https://docs.github.com/rest/reference/apps#get-the-authenticated-app
    ///
    /// Get the authenticated app
    /// Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the "[List installations for the authenticated app](https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app)" endpoint.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetApp(),
    /// * tags apps
    /// * post `/app-manifests/{code}/conversions`
    /// * docs https://docs.github.com/rest/reference/apps#create-a-github-app-from-a-manifest
    ///
    /// Create a GitHub App from a manifest
    /// Use this endpoint to complete the handshake necessary when implementing the [GitHub App Manifest flow](https://docs.github.com/apps/building-github-apps/creating-github-apps-from-a-manifest/). When you create a GitHub App with the manifest flow, you receive a temporary `code` used to retrieve the GitHub App's `id`, `pem` (private key), and `webhook_secret`.
    PostAppManifestscodeConversions(String),
    /// * tags apps
    /// * get `/app/hook/config`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-webhook-configuration-for-an-app
    ///
    /// Get a webhook configuration for an app
    /// Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetAppHookConfig(),
    /// * tags apps
    /// * patch `/app/hook/config`
    /// * docs https://docs.github.com/rest/reference/apps#update-a-webhook-configuration-for-an-app
    ///
    /// Update a webhook configuration for an app
    /// Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    PatchAppHookConfig(),
    /// * tags apps
    /// * get `/app/hook/deliveries`
    /// * docs https://docs.github.com/rest/reference/apps#list-deliveries-for-an-app-webhook
    ///
    /// List deliveries for an app webhook
    /// Returns a list of webhook deliveries for the webhook configured for a GitHub App.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetAppHookDeliveries(),
    /// * tags apps
    /// * get `/app/hook/deliveries/{delivery_id}`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-delivery-for-an-app-webhook
    ///
    /// Get a delivery for an app webhook
    /// Returns a delivery for the webhook configured for a GitHub App.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetAppHookDeliveriesdeliveryId(String),
    /// * tags apps
    /// * post `/app/hook/deliveries/{delivery_id}/attempts`
    /// * docs https://docs.github.com/rest/reference/apps#redeliver-a-delivery-for-an-app-webhook
    ///
    /// Redeliver a delivery for an app webhook
    /// Redeliver a delivery for the webhook configured for a GitHub App.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    PostAppHookDeliveriesdeliveryIdAttempts(String),
    /// * tags apps
    /// * get `/app/installations`
    /// * docs https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app
    ///
    /// List installations for the authenticated app
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    ///
    /// The permissions the installation has are included under the `permissions` key.
    GetAppInstallations(),
    /// * tags apps
    /// * get `/app/installations/{installation_id}`
    /// * docs https://docs.github.com/rest/reference/apps#get-an-installation-for-the-authenticated-app
    ///
    /// Get an installation for the authenticated app
    /// Enables an authenticated GitHub App to find an installation's information using the installation id. The installation's account type (`target_type`) will be either an organization or a user account, depending which account the repository belongs to.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetAppInstallationsinstallationId(String),
    /// * tags apps
    /// * delete `/app/installations/{installation_id}`
    /// * docs https://docs.github.com/rest/reference/apps#delete-an-installation-for-the-authenticated-app
    ///
    /// Delete an installation for the authenticated app
    /// Uninstalls a GitHub App on a user, organization, or business account. If you prefer to temporarily suspend an app's access to your account's resources, then we recommend the "[Suspend an app installation](https://docs.github.com/rest/reference/apps/#suspend-an-app-installation)" endpoint.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    DeleteAppInstallationsinstallationId(String),
    /// * tags apps
    /// * post `/app/installations/{installation_id}/access_tokens`
    /// * docs https://docs.github.com/rest/reference/apps/#create-an-installation-access-token-for-an-app
    ///
    /// Create an installation access token for an app
    /// Creates an installation access token that enables a GitHub App to make authenticated API requests for the app's installation on an organization or individual account. Installation tokens expire one hour from the time you create them. Using an expired token produces a status code of `401 - Unauthorized`, and requires creating a new installation token. By default the installation token has access to all repositories that the installation can access. To restrict the access to specific repositories, you can provide the `repository_ids` when creating the token. When you omit `repository_ids`, the response does not contain the `repositories` key.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    PostAppInstallationsinstallationIdAccessTokens(String),
    /// * tags apps
    /// * put `/app/installations/{installation_id}/suspended`
    /// * docs https://docs.github.com/rest/reference/apps#suspend-an-app-installation
    ///
    /// Suspend an app installation
    /// Suspends a GitHub App on a user, organization, or business account, which blocks the app from accessing the account's resources. When a GitHub App is suspended, the app's access to the GitHub API or webhook events is blocked for that account.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    PutAppInstallationsinstallationIdSuspended(String),
    /// * tags apps
    /// * delete `/app/installations/{installation_id}/suspended`
    /// * docs https://docs.github.com/rest/reference/apps#unsuspend-an-app-installation
    ///
    /// Unsuspend an app installation
    /// Removes a GitHub App installation suspension.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    DeleteAppInstallationsinstallationIdSuspended(String),
    /// * tags oauth-authorizations
    /// * get `/applications/grants`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#list-your-grants
    ///
    /// List your grants
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// You can use this API to list the set of OAuth applications that have been granted access to your account. Unlike the [list your authorizations](https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations) API, this API does not manage individual tokens. This API will return one entry for each OAuth application that has been granted access to your account, regardless of the number of tokens an application has generated for your user. The list of OAuth applications returned matches what is shown on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized). The `scopes` returned are the union of scopes authorized for the application. For example, if an application has one token with `repo` scope and another token with `user` scope, the grant will return `["repo", "user"]`.
    GetApplicationsGrants(),
    /// * tags oauth-authorizations
    /// * get `/applications/grants/{grant_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-grant
    ///
    /// Get a single grant
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    GetApplicationsGrantsgrantId(String),
    /// * tags oauth-authorizations
    /// * delete `/applications/grants/{grant_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#delete-a-grant
    ///
    /// Delete a grant
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// Deleting an OAuth application's grant will also delete all OAuth tokens associated with the application for your user. Once deleted, the application has no access to your account and is no longer listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
    DeleteApplicationsGrantsgrantId(String),
    /// * tags apps
    /// * delete `/applications/{client_id}/grant`
    /// * docs https://docs.github.com/rest/reference/apps#delete-an-app-authorization
    ///
    /// Delete an app authorization
    /// OAuth application owners can revoke a grant for their OAuth application and a specific user. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. You must also provide a valid OAuth `access_token` as an input parameter and the grant for the token's owner will be deleted.
    /// Deleting an OAuth application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
    DeleteApplicationsclientIdGrant(String),
    /// * tags apps
    /// * post `/applications/{client_id}/token`
    /// * docs https://docs.github.com/rest/reference/apps#check-a-token
    ///
    /// Check a token
    /// OAuth applications can use a special API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) to use this endpoint, where the username is the OAuth application `client_id` and the password is its `client_secret`. Invalid tokens will return `404 NOT FOUND`.
    PostApplicationsclientIdToken(String),
    /// * tags apps
    /// * patch `/applications/{client_id}/token`
    /// * docs https://docs.github.com/rest/reference/apps#reset-a-token
    ///
    /// Reset a token
    /// OAuth applications can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
    PatchApplicationsclientIdToken(String),
    /// * tags apps
    /// * delete `/applications/{client_id}/token`
    /// * docs https://docs.github.com/rest/reference/apps#delete-an-app-token
    ///
    /// Delete an app token
    /// OAuth application owners can revoke a single token for an OAuth application. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password.
    DeleteApplicationsclientIdToken(String),
    /// * tags apps
    /// * post `/applications/{client_id}/token/scoped`
    /// * docs https://docs.github.com/rest/reference/apps#create-a-scoped-access-token
    ///
    /// Create a scoped access token
    /// Use a non-scoped user-to-server OAuth access token to create a repository scoped and/or permission scoped user-to-server OAuth access token. You can specify which repositories the token can access and which permissions are granted to the token. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
    PostApplicationsclientIdTokenScoped(String),
    /// * tags apps
    /// * get `/apps/{app_slug}`
    /// * docs https://docs.github.com/rest/reference/apps/#get-an-app
    ///
    /// Get an app
    /// **Note**: The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).
    ///
    /// If the GitHub App you specify is public, you can access this endpoint without authenticating. If the GitHub App you specify is private, you must authenticate with a [personal access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) or an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
    GetAppsappSlug(String),
    /// * tags oauth-authorizations
    /// * get `/authorizations`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations
    ///
    /// List your authorizations
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    GetAuthorizations(),
    /// * tags oauth-authorizations
    /// * post `/authorizations`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#create-a-new-authorization
    ///
    /// Create a new authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
    ///
    /// Creates OAuth tokens using [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication). If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    ///
    /// To create tokens for a particular OAuth application using this endpoint, you must authenticate as the user you want to create an authorization for and provide the app's client ID and secret, found on your OAuth application's settings page. If your OAuth application intends to create multiple tokens for one user, use `fingerprint` to differentiate between them.
    ///
    /// You can also create tokens on GitHub from the [personal access tokens settings](https://github.com/settings/tokens) page. Read more about these tokens in [the GitHub Help documentation](https://help.github.com/articles/creating-an-access-token-for-command-line-use).
    ///
    /// Organizations that enforce SAML SSO require personal access tokens to be allowed. Read more about allowing tokens in [the GitHub Help documentation](https://help.github.com/articles/about-identity-and-access-management-with-saml-single-sign-on).
    PostAuthorizations(),
    /// * tags oauth-authorizations
    /// * put `/authorizations/clients/{client_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app
    ///
    /// Get-or-create an authorization for a specific app
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
    ///
    /// Creates a new authorization for the specified OAuth application, only if an authorization for that application doesn't already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    ///
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    PutAuthorizationsClientsclientId(String),
    /// * tags oauth-authorizations
    /// * put `/authorizations/clients/{client_id}/{fingerprint}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app-and-fingerprint
    ///
    /// Get-or-create an authorization for a specific app and fingerprint
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
    ///
    /// This method will create a new authorization for the specified OAuth application, only if an authorization for that application and fingerprint do not already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. `fingerprint` is a unique string to distinguish an authorization from others created for the same client ID and user. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    PutAuthorizationsClientsclientIdfingerprint(String, String),
    /// * tags oauth-authorizations
    /// * get `/authorizations/{authorization_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-authorization
    ///
    /// Get a single authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    GetAuthorizationsauthorizationId(String),
    /// * tags oauth-authorizations
    /// * patch `/authorizations/{authorization_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#update-an-existing-authorization
    ///
    /// Update an existing authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    ///
    /// You can only send one of these scope keys at a time.
    PatchAuthorizationsauthorizationId(String),
    /// * tags oauth-authorizations
    /// * delete `/authorizations/{authorization_id}`
    /// * docs https://docs.github.com/rest/reference/oauth-authorizations#delete-an-authorization
    ///
    /// Delete an authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    DeleteAuthorizationsauthorizationId(String),
    /// * tags codes-of-conduct
    /// * get `/codes_of_conduct`
    /// * docs https://docs.github.com/rest/reference/codes-of-conduct#get-all-codes-of-conduct
    ///
    /// Get all codes of conduct
    ///
    GetCodesOfConduct(),
    /// * tags codes-of-conduct
    /// * get `/codes_of_conduct/{key}`
    /// * docs https://docs.github.com/rest/reference/codes-of-conduct#get-a-code-of-conduct
    ///
    /// Get a code of conduct
    ///
    GetCodesOfConductkey(String),
    /// * tags emojis
    /// * get `/emojis`
    /// * docs https://docs.github.com/rest/reference/emojis#get-emojis
    ///
    /// Get emojis
    /// Lists all the emojis available to use on GitHub.
    GetEmojis(),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-github-actions-permissions-for-an-enterprise
    ///
    /// Get GitHub Actions permissions for an enterprise
    /// Gets the GitHub Actions permissions policy for organizations and allowed actions in an enterprise.
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsPermissions(String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-github-actions-permissions-for-an-enterprise
    ///
    /// Set GitHub Actions permissions for an enterprise
    /// Sets the GitHub Actions permissions policy for organizations and allowed actions in an enterprise.
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsPermissions(String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions/organizations`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-selected-organizations-enabled-for-github-actions-in-an-enterprise
    ///
    /// List selected organizations enabled for GitHub Actions in an enterprise
    /// Lists the organizations that are selected to have GitHub Actions enabled in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsPermissionsOrganizations(String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/permissions/organizations`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-selected-organizations-enabled-for-github-actions-in-an-enterprise
    ///
    /// Set selected organizations enabled for GitHub Actions in an enterprise
    /// Replaces the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsPermissionsOrganizations(String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/permissions/organizations/{org_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#enable-a-selected-organization-for-github-actions-in-an-enterprise
    ///
    /// Enable a selected organization for GitHub Actions in an enterprise
    /// Adds an organization to the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsPermissionsOrganizationsorgId(String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/permissions/organizations/{org_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#disable-a-selected-organization-for-github-actions-in-an-enterprise
    ///
    /// Disable a selected organization for GitHub Actions in an enterprise
    /// Removes an organization from the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsPermissionsOrganizationsorgId(String, String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-allowed-actions-for-an-enterprise
    ///
    /// Get allowed actions for an enterprise
    /// Gets the selected actions that are allowed in an enterprise. To use this endpoint, the enterprise permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsPermissionsSelectedActions(String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-allowed-actions-for-an-enterprise
    ///
    /// Set allowed actions for an enterprise
    /// Sets the actions that are allowed in an enterprise. To use this endpoint, the enterprise permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsPermissionsSelectedActions(String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runner-groups-for-an-enterprise
    ///
    /// List self-hosted runner groups for an enterprise
    /// Lists all self-hosted runner groups for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnerGroups(String),
    /// * tags enterprise-admin
    /// * post `/enterprises/{enterprise}/actions/runner-groups`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#create-self-hosted-runner-group-for-an-enterprise
    ///
    /// Create a self-hosted runner group for an enterprise
    /// Creates a new self-hosted runner group for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PostEnterprisesenterpriseActionsRunnerGroups(String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-group-for-an-enterprise
    ///
    /// Get a self-hosted runner group for an enterprise
    /// Gets a specific self-hosted runner group for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags enterprise-admin
    /// * patch `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#update-a-self-hosted-runner-group-for-an-enterprise
    ///
    /// Update a self-hosted runner group for an enterprise
    /// Updates the `name` and `visibility` of a self-hosted runner group in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#delete-a-self-hosted-runner-group-from-an-enterprise
    ///
    /// Delete a self-hosted runner group from an enterprise
    /// Deletes a self-hosted runner group for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-organization-access-to-a-self-hosted-runner-group-in-a-enterprise
    ///
    /// List organization access to a self-hosted runner group in an enterprise
    /// Lists the organizations with access to a self-hosted runner group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(String, String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-organization-access-to-a-self-hosted-runner-group-in-an-enterprise
    ///
    /// Set organization access for a self-hosted runner group in an enterprise
    /// Replaces the list of organizations that have access to a self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(String, String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#add-organization-access-to-a-self-hosted-runner-group-in-an-enterprise
    ///
    /// Add organization access to a self-hosted runner group in an enterprise
    /// Adds an organization to the list of selected organizations that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an enterprise](#create-a-self-hosted-runner-group-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(
        String,
        String,
        String,
    ),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#remove-organization-access-to-a-self-hosted-runner-group-in-an-enterprise
    ///
    /// Remove organization access to a self-hosted runner group in an enterprise
    /// Removes an organization from the list of selected organizations that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an enterprise](#create-a-self-hosted-runner-group-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(
        String,
        String,
        String,
    ),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-in-a-group-for-an-enterprise
    ///
    /// List self-hosted runners in a group for an enterprise
    /// Lists the self-hosted runners that are in a specific enterprise group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(String, String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-self-hosted-runners-in-a-group-for-an-enterprise
    ///
    /// Set self-hosted runners in a group for an enterprise
    /// Replaces the list of self-hosted runners that are part of an enterprise runner group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(String, String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#add-a-self-hosted-runner-to-a-group-for-an-enterprise
    ///
    /// Add a self-hosted runner to a group for an enterprise
    /// Adds a self-hosted runner to a runner group configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise`
    /// scope to use this endpoint.
    PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(String, String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#remove-a-self-hosted-runner-from-a-group-for-an-enterprise
    ///
    /// Remove a self-hosted runner from a group for an enterprise
    /// Removes a self-hosted runner from a group configured in an enterprise. The runner is then returned to the default group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(
        String,
        String,
        String,
    ),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-for-an-enterprise
    ///
    /// List self-hosted runners for an enterprise
    /// Lists all self-hosted runners configured for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunners(String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners/downloads`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-runner-applications-for-an-enterprise
    ///
    /// List runner applications for an enterprise
    /// Lists binaries for the runner application that you can download and run.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnersDownloads(String),
    /// * tags enterprise-admin
    /// * post `/enterprises/{enterprise}/actions/runners/registration-token`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#create-a-registration-token-for-an-enterprise
    ///
    /// Create a registration token for an enterprise
    /// Returns a token that you can pass to the `config` script. The token expires after one hour.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    ///
    /// #### Example using registration token
    ///
    /// Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
    ///
    /// ```
    /// ./config.sh --url https://github.com/enterprises/octo-enterprise --token TOKEN
    /// ```
    PostEnterprisesenterpriseActionsRunnersRegistrationToken(String),
    /// * tags enterprise-admin
    /// * post `/enterprises/{enterprise}/actions/runners/remove-token`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#create-a-remove-token-for-an-enterprise
    ///
    /// Create a remove token for an enterprise
    /// Returns a token that you can pass to the `config` script to remove a self-hosted runner from an enterprise. The token expires after one hour.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    ///
    /// #### Example using remove token
    ///
    /// To remove your self-hosted runner from an enterprise, replace `TOKEN` with the remove token provided by this
    /// endpoint.
    ///
    /// ```
    /// ./config.sh remove --token TOKEN
    /// ```
    PostEnterprisesenterpriseActionsRunnersRemoveToken(String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-for-an-enterprise
    ///
    /// Get a self-hosted runner for an enterprise
    /// Gets a specific self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnersrunnerId(String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#delete-self-hosted-runner-from-an-enterprise
    ///
    /// Delete a self-hosted runner from an enterprise
    /// Forces the removal of a self-hosted runner from an enterprise. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnersrunnerId(String, String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-labels-for-a-self-hosted-runner-for-an-enterprise
    ///
    /// List labels for a self-hosted runner for an enterprise
    /// Lists all labels for a self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(String, String),
    /// * tags enterprise-admin
    /// * post `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#add-custom-labels-to-a-self-hosted-runner-for-an-enterprise
    ///
    /// Add custom labels to a self-hosted runner for an enterprise
    /// Add custom labels to a self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(String, String),
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-custom-labels-for-a-self-hosted-runner-for-an-enterprise
    ///
    /// Set custom labels for a self-hosted runner for an enterprise
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#remove-all-custom-labels-from-a-self-hosted-runner-for-an-enterprise
    ///
    /// Remove all custom labels from a self-hosted runner for an enterprise
    /// Remove all custom labels from a self-hosted runner configured in an
    /// enterprise. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(String, String),
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runners/{runner_id}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#remove-a-custom-label-from-a-self-hosted-runner-for-an-enterprise
    ///
    /// Remove a custom label from a self-hosted runner for an enterprise
    /// Remove a custom label from a self-hosted runner configured
    /// in an enterprise. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(String, String, String),
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/audit-log`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-the-audit-log-for-an-enterprise
    ///
    /// Get the audit log for an enterprise
    /// Gets the audit log for an enterprise. To use this endpoint, you must be an enterprise admin, and you must use an access token with the `admin:enterprise` scope.
    GetEnterprisesenterpriseAuditLog(String),
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/actions`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-enterprise
    ///
    /// Get GitHub Actions billing for an enterprise
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// The authenticated user must be an enterprise admin.
    GetEnterprisesenterpriseSettingsBillingActions(String),
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/advanced-security`
    /// * docs https://docs.github.com/rest/reference/billing#export-advanced-security-active-committers-data-for-enterprise
    ///
    /// Get GitHub Advanced Security active committers for an enterprise
    /// Gets the GitHub Advanced Security active committers for an enterprise per repository.
    /// Each distinct user login across all repositories is counted as a single Advanced Security seat, so the total_advanced_security_committers is not the sum of active_users for each repository.
    GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(String),
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/packages`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-enterprise
    ///
    /// Get GitHub Packages billing for an enterprise
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// The authenticated user must be an enterprise admin.
    GetEnterprisesenterpriseSettingsBillingPackages(String),
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/shared-storage`
    /// * docs https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-enterprise
    ///
    /// Get shared storage billing for an enterprise
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// The authenticated user must be an enterprise admin.
    GetEnterprisesenterpriseSettingsBillingSharedStorage(String),
    /// * tags activity
    /// * get `/events`
    /// * docs https://docs.github.com/rest/reference/activity#list-public-events
    ///
    /// List public events
    /// We delay the public events feed by five minutes, which means the most recent event returned by the public events API actually occurred at least five minutes ago.
    GetEvents(),
    /// * tags activity
    /// * get `/feeds`
    /// * docs https://docs.github.com/rest/reference/activity#get-feeds
    ///
    /// Get feeds
    /// GitHub provides several timeline resources in [Atom](http://en.wikipedia.org/wiki/Atom_(standard)) format. The Feeds API lists all the feeds available to the authenticated user:
    ///
    /// *   **Timeline**: The GitHub global public timeline
    /// *   **User**: The public timeline for any user, using [URI template](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia)
    /// *   **Current user public**: The public timeline for the authenticated user
    /// *   **Current user**: The private timeline for the authenticated user
    /// *   **Current user actor**: The private timeline for activity created by the authenticated user
    /// *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of.
    /// *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.
    ///
    /// **Note**: Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) since current feed URIs use the older, non revocable auth tokens.
    GetFeeds(),
    /// * tags gists
    /// * get `/gists`
    /// * docs https://docs.github.com/rest/reference/gists#list-gists-for-the-authenticated-user
    ///
    /// List gists for the authenticated user
    /// Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
    GetGists(),
    /// * tags gists
    /// * post `/gists`
    /// * docs https://docs.github.com/rest/reference/gists#create-a-gist
    ///
    /// Create a gist
    /// Allows you to add a new gist with one or more files.
    ///
    /// **Note:** Don't name your files "gistfile" with a numerical suffix. This is the format of the automatic naming scheme that Gist uses internally.
    PostGists(),
    /// * tags gists
    /// * get `/gists/public`
    /// * docs https://docs.github.com/rest/reference/gists#list-public-gists
    ///
    /// List public gists
    /// List public gists sorted by most recently updated to least recently updated.
    ///
    /// Note: With [pagination](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.
    GetGistsPublic(),
    /// * tags gists
    /// * get `/gists/starred`
    /// * docs https://docs.github.com/rest/reference/gists#list-starred-gists
    ///
    /// List starred gists
    /// List the authenticated user's starred gists:
    GetGistsStarred(),
    /// * tags gists
    /// * get `/gists/{gist_id}`
    /// * docs https://docs.github.com/rest/reference/gists#get-a-gist
    ///
    /// Get a gist
    ///
    GetGistsgistId(String),
    /// * tags gists
    /// * patch `/gists/{gist_id}`
    /// * docs https://docs.github.com/rest/reference/gists/#update-a-gist
    ///
    /// Update a gist
    /// Allows you to update or delete a gist file and rename gist files. Files from the previous version of the gist that aren't explicitly changed during an edit are unchanged.
    PatchGistsgistId(String),
    /// * tags gists
    /// * delete `/gists/{gist_id}`
    /// * docs https://docs.github.com/rest/reference/gists#delete-a-gist
    ///
    /// Delete a gist
    ///
    DeleteGistsgistId(String),
    /// * tags gists
    /// * get `/gists/{gist_id}/comments`
    /// * docs https://docs.github.com/rest/reference/gists#list-gist-comments
    ///
    /// List gist comments
    ///
    GetGistsgistIdComments(String),
    /// * tags gists
    /// * post `/gists/{gist_id}/comments`
    /// * docs https://docs.github.com/rest/reference/gists#create-a-gist-comment
    ///
    /// Create a gist comment
    ///
    PostGistsgistIdComments(String),
    /// * tags gists
    /// * get `/gists/{gist_id}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/gists#get-a-gist-comment
    ///
    /// Get a gist comment
    ///
    GetGistsgistIdCommentscommentId(String, String),
    /// * tags gists
    /// * patch `/gists/{gist_id}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/gists#update-a-gist-comment
    ///
    /// Update a gist comment
    ///
    PatchGistsgistIdCommentscommentId(String, String),
    /// * tags gists
    /// * delete `/gists/{gist_id}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/gists#delete-a-gist-comment
    ///
    /// Delete a gist comment
    ///
    DeleteGistsgistIdCommentscommentId(String, String),
    /// * tags gists
    /// * get `/gists/{gist_id}/commits`
    /// * docs https://docs.github.com/rest/reference/gists#list-gist-commits
    ///
    /// List gist commits
    ///
    GetGistsgistIdCommits(String),
    /// * tags gists
    /// * get `/gists/{gist_id}/forks`
    /// * docs https://docs.github.com/rest/reference/gists#list-gist-forks
    ///
    /// List gist forks
    ///
    GetGistsgistIdForks(String),
    /// * tags gists
    /// * post `/gists/{gist_id}/forks`
    /// * docs https://docs.github.com/rest/reference/gists#fork-a-gist
    ///
    /// Fork a gist
    /// **Note**: This was previously `/gists/:gist_id/fork`.
    PostGistsgistIdForks(String),
    /// * tags gists
    /// * get `/gists/{gist_id}/star`
    /// * docs https://docs.github.com/rest/reference/gists#check-if-a-gist-is-starred
    ///
    /// Check if a gist is starred
    ///
    GetGistsgistIdStar(String),
    /// * tags gists
    /// * put `/gists/{gist_id}/star`
    /// * docs https://docs.github.com/rest/reference/gists#star-a-gist
    ///
    /// Star a gist
    /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutGistsgistIdStar(String),
    /// * tags gists
    /// * delete `/gists/{gist_id}/star`
    /// * docs https://docs.github.com/rest/reference/gists#unstar-a-gist
    ///
    /// Unstar a gist
    ///
    DeleteGistsgistIdStar(String),
    /// * tags gists
    /// * get `/gists/{gist_id}/{sha}`
    /// * docs https://docs.github.com/rest/reference/gists#get-a-gist-revision
    ///
    /// Get a gist revision
    ///
    GetGistsgistIdsha(String, String),
    /// * tags gitignore
    /// * get `/gitignore/templates`
    /// * docs https://docs.github.com/rest/reference/gitignore#get-all-gitignore-templates
    ///
    /// Get all gitignore templates
    /// List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user).
    GetGitignoreTemplates(),
    /// * tags gitignore
    /// * get `/gitignore/templates/{name}`
    /// * docs https://docs.github.com/rest/reference/gitignore#get-a-gitignore-template
    ///
    /// Get a gitignore template
    /// The API also allows fetching the source of a single template.
    /// Use the raw [media type](https://docs.github.com/rest/overview/media-types/) to get the raw contents.
    GetGitignoreTemplatesname(String),
    /// * tags apps
    /// * get `/installation/repositories`
    /// * docs https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-app-installation
    ///
    /// List repositories accessible to the app installation
    /// List repositories that an app installation can access.
    ///
    /// You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
    GetInstallationRepositories(),
    /// * tags apps
    /// * delete `/installation/token`
    /// * docs https://docs.github.com/rest/reference/apps#revoke-an-installation-access-token
    ///
    /// Revoke an installation access token
    /// Revokes the installation token you're using to authenticate as an installation and access this endpoint.
    ///
    /// Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the "[Create an installation access token for an app](https://docs.github.com/rest/reference/apps#create-an-installation-access-token-for-an-app)" endpoint.
    ///
    /// You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
    DeleteInstallationToken(),
    /// * tags issues
    /// * get `/issues`
    /// * docs https://docs.github.com/rest/reference/issues#list-issues-assigned-to-the-authenticated-user
    ///
    /// List issues assigned to the authenticated user
    /// List issues assigned to the authenticated user across all visible repositories including owned repositories, member
    /// repositories, and organization repositories. You can use the `filter` query parameter to fetch issues that are not
    /// necessarily assigned to you.
    ///
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetIssues(),
    /// * tags licenses
    /// * get `/licenses`
    /// * docs https://docs.github.com/rest/reference/licenses#get-all-commonly-used-licenses
    ///
    /// Get all commonly used licenses
    ///
    GetLicenses(),
    /// * tags licenses
    /// * get `/licenses/{license}`
    /// * docs https://docs.github.com/rest/reference/licenses#get-a-license
    ///
    /// Get a license
    ///
    GetLicenseslicense(String),
    /// * tags markdown
    /// * post `/markdown`
    /// * docs https://docs.github.com/rest/reference/markdown#render-a-markdown-document
    ///
    /// Render a Markdown document
    ///
    PostMarkdown(),
    /// * tags markdown
    /// * post `/markdown/raw`
    /// * docs https://docs.github.com/rest/reference/markdown#render-a-markdown-document-in-raw-mode
    ///
    /// Render a Markdown document in raw mode
    /// You must send Markdown as plain text (using a `Content-Type` header of `text/plain` or `text/x-markdown`) to this endpoint, rather than using JSON format. In raw mode, [GitHub Flavored Markdown](https://github.github.com/gfm/) is not supported and Markdown will be rendered in plain format like a README.md file. Markdown content must be 400 KB or less.
    PostMarkdownRaw(),
    /// * tags apps
    /// * get `/marketplace_listing/accounts/{account_id}`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account
    ///
    /// Get a subscription plan for an account
    /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingAccountsaccountId(String),
    /// * tags apps
    /// * get `/marketplace_listing/plans`
    /// * docs https://docs.github.com/rest/reference/apps#list-plans
    ///
    /// List plans
    /// Lists all plans that are part of your GitHub Marketplace listing.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingPlans(),
    /// * tags apps
    /// * get `/marketplace_listing/plans/{plan_id}/accounts`
    /// * docs https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan
    ///
    /// List accounts for a plan
    /// Returns user and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingPlansplanIdAccounts(String),
    /// * tags apps
    /// * get `/marketplace_listing/stubbed/accounts/{account_id}`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account-stubbed
    ///
    /// Get a subscription plan for an account (stubbed)
    /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingStubbedAccountsaccountId(String),
    /// * tags apps
    /// * get `/marketplace_listing/stubbed/plans`
    /// * docs https://docs.github.com/rest/reference/apps#list-plans-stubbed
    ///
    /// List plans (stubbed)
    /// Lists all plans that are part of your GitHub Marketplace listing.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingStubbedPlans(),
    /// * tags apps
    /// * get `/marketplace_listing/stubbed/plans/{plan_id}/accounts`
    /// * docs https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan-stubbed
    ///
    /// List accounts for a plan (stubbed)
    /// Returns repository and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    GetMarketplaceListingStubbedPlansplanIdAccounts(String),
    /// * tags meta
    /// * get `/meta`
    /// * docs https://docs.github.com/rest/reference/meta#get-github-meta-information
    ///
    /// Get GitHub meta information
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://help.github.com/articles/about-github-s-ip-addresses/)."
    ///
    /// **Note:** The IP addresses shown in the documentation's response are only example values. You must always query the API directly to get the latest list of IP addresses.
    GetMeta(),
    /// * tags activity
    /// * get `/networks/{owner}/{repo}/events`
    /// * docs https://docs.github.com/rest/reference/activity#list-public-events-for-a-network-of-repositories
    ///
    /// List public events for a network of repositories
    ///
    GetNetworksownerrepoEvents(String, String),
    /// * tags activity
    /// * get `/notifications`
    /// * docs https://docs.github.com/rest/reference/activity#list-notifications-for-the-authenticated-user
    ///
    /// List notifications for the authenticated user
    /// List all notifications for the current user, sorted by most recently updated.
    GetNotifications(),
    /// * tags activity
    /// * put `/notifications`
    /// * docs https://docs.github.com/rest/reference/activity#mark-notifications-as-read
    ///
    /// Mark notifications as read
    /// Marks all notifications as "read" removes it from the [default view on GitHub](https://github.com/notifications). If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List notifications for the authenticated user](https://docs.github.com/rest/reference/activity#list-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
    PutNotifications(),
    /// * tags activity
    /// * get `/notifications/threads/{thread_id}`
    /// * docs https://docs.github.com/rest/reference/activity#get-a-thread
    ///
    /// Get a thread
    ///
    GetNotificationsThreadsthreadId(String),
    /// * tags activity
    /// * patch `/notifications/threads/{thread_id}`
    /// * docs https://docs.github.com/rest/reference/activity#mark-a-thread-as-read
    ///
    /// Mark a thread as read
    ///
    PatchNotificationsThreadsthreadId(String),
    /// * tags activity
    /// * get `/notifications/threads/{thread_id}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#get-a-thread-subscription-for-the-authenticated-user
    ///
    /// Get a thread subscription for the authenticated user
    /// This checks to see if the current user is subscribed to a thread. You can also [get a repository subscription](https://docs.github.com/rest/reference/activity#get-a-repository-subscription).
    ///
    /// Note that subscriptions are only generated if a user is participating in a conversation--for example, they've replied to the thread, were **@mentioned**, or manually subscribe to a thread.
    GetNotificationsThreadsthreadIdSubscription(String),
    /// * tags activity
    /// * put `/notifications/threads/{thread_id}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#set-a-thread-subscription
    ///
    /// Set a thread subscription
    /// If you are watching a repository, you receive notifications for all threads by default. Use this endpoint to ignore future notifications for threads until you comment on the thread or get an **@mention**.
    ///
    /// You can also use this endpoint to subscribe to threads that you are currently not receiving notifications for or to subscribed to threads that you have previously ignored.
    ///
    /// Unsubscribing from a conversation in a repository that you are not watching is functionally equivalent to the [Delete a thread subscription](https://docs.github.com/rest/reference/activity#delete-a-thread-subscription) endpoint.
    PutNotificationsThreadsthreadIdSubscription(String),
    /// * tags activity
    /// * delete `/notifications/threads/{thread_id}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#delete-a-thread-subscription
    ///
    /// Delete a thread subscription
    /// Mutes all future notifications for a conversation until you comment on the thread or get an **@mention**. If you are watching the repository of the thread, you will still receive notifications. To ignore future notifications for a repository you are watching, use the [Set a thread subscription](https://docs.github.com/rest/reference/activity#set-a-thread-subscription) endpoint and set `ignore` to `true`.
    DeleteNotificationsThreadsthreadIdSubscription(String),
    /// * tags meta
    /// * get `/octocat`
    /// * docs https://docs.github.com/rest/reference/meta#get-octocat
    ///
    /// Get Octocat
    /// Get the octocat as ASCII art
    GetOctocat(),
    /// * tags orgs
    /// * get `/organizations`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organizations
    ///
    /// List organizations
    /// Lists all organizations, in the order that they were created on GitHub.
    ///
    /// **Note:** Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of organizations.
    GetOrganizations(),
    /// * tags orgs
    /// * get `/organizations/{organization_id}/custom_roles`
    /// * docs https://docs.github.com/rest/reference/orgs#list-custom-repository-roles-in-an-organization
    ///
    /// List custom repository roles in an organization
    /// List the custom repository roles available in this organization. In order to see custom
    /// repository roles in an organization, the authenticated user must be an organization owner.
    ///
    /// For more information on custom repository roles, see "[Managing custom repository roles for an organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-custom-repository-roles-for-an-organization)".
    GetOrganizationsorganizationIdCustomRoles(String),
    /// * tags orgs
    /// * get `/orgs/{org}`
    /// * docs https://docs.github.com/rest/reference/orgs#get-an-organization
    ///
    /// Get an organization
    /// To see many of the organization response values, you need to be an authenticated organization owner with the `admin:org` scope. When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://help.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
    ///
    /// GitHub Apps with the `Organization plan` permission can use this endpoint to retrieve information about an organization's GitHub plan. See "[Authenticating with GitHub Apps](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/)" for details. For an example response, see 'Response with GitHub plan information' below."
    GetOrgsorg(String),
    /// * tags orgs
    /// * patch `/orgs/{org}`
    /// * docs https://docs.github.com/rest/reference/orgs/#update-an-organization
    ///
    /// Update an organization
    /// **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
    ///
    /// Enables an authenticated organization owner with the `admin:org` scope to update the organization's profile and member privileges.
    PatchOrgsorg(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-an-organization
    ///
    /// Get GitHub Actions permissions for an organization
    /// Gets the GitHub Actions permissions policy for repositories and allowed actions in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    GetOrgsorgActionsPermissions(String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/actions#set-github-actions-permissions-for-an-organization
    ///
    /// Set GitHub Actions permissions for an organization
    /// Sets the GitHub Actions permissions policy for repositories and allowed actions in an organization.
    ///
    /// If the organization belongs to an enterprise that has set restrictive permissions at the enterprise level, such as `allowed_actions` to `selected` actions, then you cannot override them for the organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    PutOrgsorgActionsPermissions(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#list-selected-repositories-enabled-for-github-actions-in-an-organization
    ///
    /// List selected repositories enabled for GitHub Actions in an organization
    /// Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    GetOrgsorgActionsPermissionsRepositories(String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/permissions/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#set-selected-repositories-enabled-for-github-actions-in-an-organization
    ///
    /// Set selected repositories enabled for GitHub Actions in an organization
    /// Replaces the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    PutOrgsorgActionsPermissionsRepositories(String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/permissions/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#enable-a-selected-repository-for-github-actions-in-an-organization
    ///
    /// Enable a selected repository for GitHub Actions in an organization
    /// Adds a repository to the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    PutOrgsorgActionsPermissionsRepositoriesrepositoryId(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/permissions/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#disable-a-selected-repository-for-github-actions-in-an-organization
    ///
    /// Disable a selected repository for GitHub Actions in an organization
    /// Removes a repository from the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    DeleteOrgsorgActionsPermissionsRepositoriesrepositoryId(String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/actions#get-allowed-actions-for-an-organization
    ///
    /// Get allowed actions for an organization
    /// Gets the selected actions that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).""
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    GetOrgsorgActionsPermissionsSelectedActions(String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/actions#set-allowed-actions-for-an-organization
    ///
    /// Set allowed actions for an organization
    /// Sets the actions that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// If the organization belongs to an enterprise that has `selected` actions set at the enterprise level, then you cannot override any of the enterprise's allowed actions settings.
    ///
    /// To use the `patterns_allowed` setting for private repositories, the organization must belong to an enterprise. If the organization does not belong to an enterprise, then the `patterns_allowed` setting only applies to public repositories in the organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    PutOrgsorgActionsPermissionsSelectedActions(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups`
    /// * docs https://docs.github.com/rest/reference/actions#list-self-hosted-runner-groups-for-an-organization
    ///
    /// List self-hosted runner groups for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists all self-hosted runner groups configured in an organization and inherited from an enterprise.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnerGroups(String),
    /// * tags actions
    /// * post `/orgs/{org}/actions/runner-groups`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-self-hosted-runner-group-for-an-organization
    ///
    /// Create a self-hosted runner group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud and GitHub Enterprise Server. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Creates a new self-hosted runner group for an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PostOrgsorgActionsRunnerGroups(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-group-for-an-organization
    ///
    /// Get a self-hosted runner group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Gets a specific self-hosted runner group for an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags actions
    /// * patch `/orgs/{org}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/actions#update-a-self-hosted-runner-group-for-an-organization
    ///
    /// Update a self-hosted runner group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Updates the `name` and `visibility` of a self-hosted runner group in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PatchOrgsorgActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runner-groups/{runner_group_id}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-group-from-an-organization
    ///
    /// Delete a self-hosted runner group from an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Deletes a self-hosted runner group for an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnerGroupsrunnerGroupId(String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#list-repository-access-to-a-self-hosted-runner-group-in-an-organization
    ///
    /// List repository access to a self-hosted runner group in an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud and GitHub Enterprise Server. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists the repositories with access to a self-hosted runner group configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#set-repository-access-to-a-self-hosted-runner-group-in-an-organization
    ///
    /// Set repository access for a self-hosted runner group in an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Replaces the list of repositories that have access to a self-hosted runner group configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#add-repository-acess-to-a-self-hosted-runner-group-in-an-organization
    ///
    /// Add repository access to a self-hosted runner group in an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    ///
    /// Adds a repository to the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org`
    /// scope to use this endpoint.
    PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(String, String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#remove-repository-access-to-a-self-hosted-runner-group-in-an-organization
    ///
    /// Remove repository access to a self-hosted runner group in an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    ///
    /// Removes a repository from the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(String, String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs https://docs.github.com/rest/reference/actions#list-self-hosted-runners-in-a-group-for-an-organization
    ///
    /// List self-hosted runners in a group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists self-hosted runners that are in a specific organization group.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs https://docs.github.com/rest/reference/actions#set-self-hosted-runners-in-a-group-for-an-organization
    ///
    /// Set self-hosted runners in a group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Replaces the list of self-hosted runners that are part of an organization runner group.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#add-a-self-hosted-runner-to-a-group-for-an-organization
    ///
    /// Add a self-hosted runner to a group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    ///
    /// Adds a self-hosted runner to a runner group configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org`
    /// scope to use this endpoint.
    PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(String, String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#remove-a-self-hosted-runner-from-a-group-for-an-organization
    ///
    /// Remove a self-hosted runner from a group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    ///
    /// Removes a self-hosted runner from a group configured in an organization. The runner is then returned to the default group.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(String, String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners`
    /// * docs https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-an-organization
    ///
    /// List self-hosted runners for an organization
    /// Lists all self-hosted runners configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunners(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners/downloads`
    /// * docs https://docs.github.com/rest/reference/actions#list-runner-applications-for-an-organization
    ///
    /// List runner applications for an organization
    /// Lists binaries for the runner application that you can download and run.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnersDownloads(String),
    /// * tags actions
    /// * post `/orgs/{org}/actions/runners/registration-token`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-registration-token-for-an-organization
    ///
    /// Create a registration token for an organization
    /// Returns a token that you can pass to the `config` script. The token expires after one hour.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    ///
    /// #### Example using registration token
    ///
    /// Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
    ///
    /// ```
    /// ./config.sh --url https://github.com/octo-org --token TOKEN
    /// ```
    PostOrgsorgActionsRunnersRegistrationToken(String),
    /// * tags actions
    /// * post `/orgs/{org}/actions/runners/remove-token`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-remove-token-for-an-organization
    ///
    /// Create a remove token for an organization
    /// Returns a token that you can pass to the `config` script to remove a self-hosted runner from an organization. The token expires after one hour.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    ///
    /// #### Example using remove token
    ///
    /// To remove your self-hosted runner from an organization, replace `TOKEN` with the remove token provided by this
    /// endpoint.
    ///
    /// ```
    /// ./config.sh remove --token TOKEN
    /// ```
    PostOrgsorgActionsRunnersRemoveToken(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-an-organization
    ///
    /// Get a self-hosted runner for an organization
    /// Gets a specific self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnersrunnerId(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-from-an-organization
    ///
    /// Delete a self-hosted runner from an organization
    /// Forces the removal of a self-hosted runner from an organization. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnersrunnerId(String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#list-labels-for-a-self-hosted-runner-for-an-organization
    ///
    /// List labels for a self-hosted runner for an organization
    /// Lists all labels for a self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    GetOrgsorgActionsRunnersrunnerIdLabels(String, String),
    /// * tags actions
    /// * post `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#add-custom-labels-to-a-self-hosted-runner-for-an-organization
    ///
    /// Add custom labels to a self-hosted runner for an organization
    /// Add custom labels to a self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PostOrgsorgActionsRunnersrunnerIdLabels(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#set-custom-labels-for-a-self-hosted-runner-for-an-organization
    ///
    /// Set custom labels for a self-hosted runner for an organization
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    PutOrgsorgActionsRunnersrunnerIdLabels(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization
    ///
    /// Remove all custom labels from a self-hosted runner for an organization
    /// Remove all custom labels from a self-hosted runner configured in an
    /// organization. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnersrunnerIdLabels(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runners/{runner_id}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/actions#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization
    ///
    /// Remove a custom label from a self-hosted runner for an organization
    /// Remove a custom label from a self-hosted runner configured
    /// in an organization. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    DeleteOrgsorgActionsRunnersrunnerIdLabelsname(String, String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets`
    /// * docs https://docs.github.com/rest/reference/actions#list-organization-secrets
    ///
    /// List organization secrets
    /// Lists all secrets available in an organization without revealing their encrypted values. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    GetOrgsorgActionsSecrets(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/public-key`
    /// * docs https://docs.github.com/rest/reference/actions#get-an-organization-public-key
    ///
    /// Get an organization public key
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    GetOrgsorgActionsSecretsPublicKey(String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#get-an-organization-secret
    ///
    /// Get an organization secret
    /// Gets a single organization secret without revealing its encrypted value. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    GetOrgsorgActionsSecretssecretName(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret
    ///
    /// Create or update an organization secret
    /// Creates or updates an organization secret with an encrypted value. Encrypt your secret using
    /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
    /// token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to
    /// use this endpoint.
    ///
    /// #### Example encrypting a secret using Node.js
    ///
    /// Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
    ///
    /// ```
    /// const sodium = require('tweetsodium');
    ///
    /// const key = "base64-encoded-public-key";
    /// const value = "plain-text-secret";
    ///
    /// // Convert the message and key to Uint8Array's (Buffer implements that interface)
    /// const messageBytes = Buffer.from(value);
    /// const keyBytes = Buffer.from(key, 'base64');
    ///
    /// // Encrypt using LibSodium.
    /// const encryptedBytes = sodium.seal(messageBytes, keyBytes);
    ///
    /// // Base64 the encrypted secret
    /// const encrypted = Buffer.from(encryptedBytes).toString('base64');
    ///
    /// console.log(encrypted);
    /// ```
    ///
    ///
    /// #### Example encrypting a secret using Python
    ///
    /// Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
    ///
    /// ```
    /// from base64 import b64encode
    /// from nacl import encoding, public
    ///
    /// def encrypt(public_key: str, secret_value: str) -> str:
    ///   """Encrypt a Unicode string using the public key."""
    ///   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
    ///   sealed_box = public.SealedBox(public_key)
    ///   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
    ///   return b64encode(encrypted).decode("utf-8")
    /// ```
    ///
    /// #### Example encrypting a secret using C#
    ///
    /// Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
    ///
    /// ```
    /// var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
    /// var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
    ///
    /// var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
    ///
    /// Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
    /// ```
    ///
    /// #### Example encrypting a secret using Ruby
    ///
    /// Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
    ///
    /// ```ruby
    /// require "rbnacl"
    /// require "base64"
    ///
    /// key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
    /// public_key = RbNaCl::PublicKey.new(key)
    ///
    /// box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
    /// encrypted_secret = box.encrypt("my_secret")
    ///
    /// # Print the base64 encoded secret
    /// puts Base64.strict_encode64(encrypted_secret)
    /// ```
    PutOrgsorgActionsSecretssecretName(String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-an-organization-secret
    ///
    /// Delete an organization secret
    /// Deletes a secret in an organization using the secret name. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    DeleteOrgsorgActionsSecretssecretName(String, String),
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/{secret_name}/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#list-selected-repositories-for-an-organization-secret
    ///
    /// List selected repositories for an organization secret
    /// Lists all repositories that have been selected when the `visibility` for repository access to a secret is set to `selected`. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    GetOrgsorgActionsSecretssecretNameRepositories(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/secrets/{secret_name}/repositories`
    /// * docs https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret
    ///
    /// Set selected repositories for an organization secret
    /// Replaces all repositories for an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    PutOrgsorgActionsSecretssecretNameRepositories(String, String),
    /// * tags actions
    /// * put `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#add-selected-repository-to-an-organization-secret
    ///
    /// Add selected repository to an organization secret
    /// Adds a repository to an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    PutOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(String, String, String),
    /// * tags actions
    /// * delete `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret
    ///
    /// Remove selected repository from an organization secret
    /// Removes a repository from an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    DeleteOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(String, String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/audit-log`
    /// * docs https://docs.github.com/rest/reference/orgs#get-audit-log
    ///
    /// Get the audit log for an organization
    /// Gets the audit log for an organization. For more information, see "[Reviewing the audit log for your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/reviewing-the-audit-log-for-your-organization)."
    ///
    /// To use this endpoint, you must be an organization owner, and you must use an access token with the `admin:org` scope. GitHub Apps must have the `organization_administration` read permission to use this endpoint.
    GetOrgsorgAuditLog(String),
    /// * tags orgs
    /// * get `/orgs/{org}/blocks`
    /// * docs https://docs.github.com/rest/reference/orgs#list-users-blocked-by-an-organization
    ///
    /// List users blocked by an organization
    /// List the users blocked by an organization.
    GetOrgsorgBlocks(String),
    /// * tags orgs
    /// * get `/orgs/{org}/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#check-if-a-user-is-blocked-by-an-organization
    ///
    /// Check if a user is blocked by an organization
    ///
    GetOrgsorgBlocksusername(String, String),
    /// * tags orgs
    /// * put `/orgs/{org}/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#block-a-user-from-an-organization
    ///
    /// Block a user from an organization
    ///
    PutOrgsorgBlocksusername(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#unblock-a-user-from-an-organization
    ///
    /// Unblock a user from an organization
    ///
    DeleteOrgsorgBlocksusername(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/credential-authorizations`
    /// * docs https://docs.github.com/rest/reference/orgs#list-saml-sso-authorizations-for-an-organization
    ///
    /// List SAML SSO authorizations for an organization
    /// Listing and deleting credential authorizations is available to organizations with GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products).
    ///
    /// An authenticated organization owner with the `read:org` scope can list all credential authorizations for an organization that uses SAML single sign-on (SSO). The credentials are either personal access tokens or SSH keys that organization members have authorized for the organization. For more information, see [About authentication with SAML single sign-on](https://help.github.com/en/articles/about-authentication-with-saml-single-sign-on).
    GetOrgsorgCredentialAuthorizations(String),
    /// * tags orgs
    /// * delete `/orgs/{org}/credential-authorizations/{credential_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#remove-a-saml-sso-authorization-for-an-organization
    ///
    /// Remove a SAML SSO authorization for an organization
    /// Listing and deleting credential authorizations is available to organizations with GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products).
    ///
    /// An authenticated organization owner with the `admin:org` scope can remove a credential authorization for an organization that uses SAML SSO. Once you remove someone's credential authorization, they will need to create a new personal access token or SSH key and authorize it for the organization they want to access.
    DeleteOrgsorgCredentialAuthorizationscredentialId(String, String),
    /// * tags activity
    /// * get `/orgs/{org}/events`
    /// * docs https://docs.github.com/rest/reference/activity#list-public-organization-events
    ///
    /// List public organization events
    ///
    GetOrgsorgEvents(String),
    /// * tags teams
    /// * get `/orgs/{org}/external-group/{group_id}`
    /// * docs https://docs.github.com/rest/reference/teams#external-idp-group-info-for-an-organization
    ///
    /// Get an external group
    /// Displays information about the specific group's usage.  Provides a list of the group's external members as well as a list of teams that this group is connected to.
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    GetOrgsorgExternalGroupgroupId(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/external-groups`
    /// * docs https://docs.github.com/rest/reference/teams#list-external-idp-groups-for-an-organization
    ///
    /// List external groups in an organization
    /// Lists external groups available in an organization. You can query the groups using the `display_name` parameter, only groups with a `group_name` containing the text provided in the `display_name` parameter will be returned.  You can also limit your page results using the `per_page` parameter. GitHub generates a url-encoded `page` token using a cursor value for where the next page begins. For more information on cursor pagination, see "[Offset and Cursor Pagination explained](https://dev.to/jackmarchant/offset-and-cursor-pagination-explained-b89)."
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    GetOrgsorgExternalGroups(String),
    /// * tags orgs
    /// * get `/orgs/{org}/failed_invitations`
    /// * docs https://docs.github.com/rest/reference/orgs#list-failed-organization-invitations
    ///
    /// List failed organization invitations
    /// The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
    GetOrgsorgFailedInvitations(String),
    /// * tags orgs
    /// * get `/orgs/{org}/hooks`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organization-webhooks
    ///
    /// List organization webhooks
    ///
    GetOrgsorgHooks(String),
    /// * tags orgs
    /// * post `/orgs/{org}/hooks`
    /// * docs https://docs.github.com/rest/reference/orgs#create-an-organization-webhook
    ///
    /// Create an organization webhook
    /// Here's how you can create a hook that posts payloads in JSON format:
    PostOrgsorgHooks(String),
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#get-an-organization-webhook
    ///
    /// Get an organization webhook
    /// Returns a webhook configured in an organization. To get only the webhook `config` properties, see "[Get a webhook configuration for an organization](/rest/reference/orgs#get-a-webhook-configuration-for-an-organization)."
    GetOrgsorgHookshookId(String, String),
    /// * tags orgs
    /// * patch `/orgs/{org}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#update-an-organization-webhook
    ///
    /// Update an organization webhook
    /// Updates a webhook configured in an organization. When you update a webhook, the `secret` will be overwritten. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for an organization](/rest/reference/orgs#update-a-webhook-configuration-for-an-organization)."
    PatchOrgsorgHookshookId(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#delete-an-organization-webhook
    ///
    /// Delete an organization webhook
    ///
    DeleteOrgsorgHookshookId(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}/config`
    /// * docs https://docs.github.com/rest/reference/orgs#get-a-webhook-configuration-for-an-organization
    ///
    /// Get a webhook configuration for an organization
    /// Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/reference/orgs#get-an-organization-webhook)."
    ///
    /// Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:read` permission.
    GetOrgsorgHookshookIdConfig(String, String),
    /// * tags orgs
    /// * patch `/orgs/{org}/hooks/{hook_id}/config`
    /// * docs https://docs.github.com/rest/reference/orgs#update-a-webhook-configuration-for-an-organization
    ///
    /// Update a webhook configuration for an organization
    /// Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/reference/orgs#update-an-organization-webhook)."
    ///
    /// Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:write` permission.
    PatchOrgsorgHookshookIdConfig(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}/deliveries`
    /// * docs https://docs.github.com/rest/reference/orgs#list-deliveries-for-an-organization-webhook
    ///
    /// List deliveries for an organization webhook
    /// Returns a list of webhook deliveries for a webhook configured in an organization.
    GetOrgsorgHookshookIdDeliveries(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#get-a-webhook-delivery-for-an-organization-webhook
    ///
    /// Get a webhook delivery for an organization webhook
    /// Returns a delivery for a webhook configured in an organization.
    GetOrgsorgHookshookIdDeliveriesdeliveryId(String, String, String),
    /// * tags orgs
    /// * post `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts`
    /// * docs https://docs.github.com/rest/reference/orgs#redeliver-a-delivery-for-an-organization-webhook
    ///
    /// Redeliver a delivery for an organization webhook
    /// Redeliver a delivery for a webhook configured in an organization.
    PostOrgsorgHookshookIdDeliveriesdeliveryIdAttempts(String, String, String),
    /// * tags orgs
    /// * post `/orgs/{org}/hooks/{hook_id}/pings`
    /// * docs https://docs.github.com/rest/reference/orgs#ping-an-organization-webhook
    ///
    /// Ping an organization webhook
    /// This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.
    PostOrgsorgHookshookIdPings(String, String),
    /// * tags apps
    /// * get `/orgs/{org}/installation`
    /// * docs https://docs.github.com/rest/reference/apps#get-an-organization-installation-for-the-authenticated-app
    ///
    /// Get an organization installation for the authenticated app
    /// Enables an authenticated GitHub App to find the organization's installation information.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetOrgsorgInstallation(String),
    /// * tags orgs
    /// * get `/orgs/{org}/installations`
    /// * docs https://docs.github.com/rest/reference/orgs#list-app-installations-for-an-organization
    ///
    /// List app installations for an organization
    /// Lists all GitHub Apps in an organization. The installation count includes all GitHub Apps installed on repositories in the organization. You must be an organization owner with `admin:read` scope to use this endpoint.
    GetOrgsorgInstallations(String),
    /// * tags interactions
    /// * get `/orgs/{org}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-an-organization
    ///
    /// Get interaction restrictions for an organization
    /// Shows which type of GitHub user can interact with this organization and when the restriction expires. If there is no restrictions, you will see an empty response.
    GetOrgsorgInteractionLimits(String),
    /// * tags interactions
    /// * put `/orgs/{org}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-an-organization
    ///
    /// Set interaction restrictions for an organization
    /// Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.
    PutOrgsorgInteractionLimits(String),
    /// * tags interactions
    /// * delete `/orgs/{org}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-for-an-organization
    ///
    /// Remove interaction restrictions for an organization
    /// Removes all interaction restrictions from public repositories in the given organization. You must be an organization owner to remove restrictions.
    DeleteOrgsorgInteractionLimits(String),
    /// * tags orgs
    /// * get `/orgs/{org}/invitations`
    /// * docs https://docs.github.com/rest/reference/orgs#list-pending-organization-invitations
    ///
    /// List pending organization invitations
    /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
    GetOrgsorgInvitations(String),
    /// * tags orgs
    /// * post `/orgs/{org}/invitations`
    /// * docs https://docs.github.com/rest/reference/orgs#create-an-organization-invitation
    ///
    /// Create an organization invitation
    /// Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostOrgsorgInvitations(String),
    /// * tags orgs
    /// * delete `/orgs/{org}/invitations/{invitation_id}`
    /// * docs https://docs.github.com/rest/reference/orgs#cancel-an-organization-invitation
    ///
    /// Cancel an organization invitation
    /// Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications).
    DeleteOrgsorgInvitationsinvitationId(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/invitations/{invitation_id}/teams`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organization-invitation-teams
    ///
    /// List organization invitation teams
    /// List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
    GetOrgsorgInvitationsinvitationIdTeams(String, String),
    /// * tags issues
    /// * get `/orgs/{org}/issues`
    /// * docs https://docs.github.com/rest/reference/issues#list-organization-issues-assigned-to-the-authenticated-user
    ///
    /// List organization issues assigned to the authenticated user
    /// List issues in an organization assigned to the authenticated user.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetOrgsorgIssues(String),
    /// * tags orgs
    /// * get `/orgs/{org}/members`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organization-members
    ///
    /// List organization members
    /// List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
    GetOrgsorgMembers(String),
    /// * tags orgs
    /// * get `/orgs/{org}/members/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#check-organization-membership-for-a-user
    ///
    /// Check organization membership for a user
    /// Check if a user is, publicly or privately, a member of the organization.
    GetOrgsorgMembersusername(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/members/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#remove-an-organization-member
    ///
    /// Remove an organization member
    /// Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.
    DeleteOrgsorgMembersusername(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user
    ///
    /// Get organization membership for a user
    /// In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
    GetOrgsorgMembershipsusername(String, String),
    /// * tags orgs
    /// * put `/orgs/{org}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#set-organization-membership-for-a-user
    ///
    /// Set organization membership for a user
    /// Only authenticated organization owners can add a member to the organization or update the member's role.
    ///
    /// *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
    ///     
    /// *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
    ///
    /// **Rate limits**
    ///
    /// To prevent abuse, the authenticated user is limited to 50 organization invitations per 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
    PutOrgsorgMembershipsusername(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#remove-organization-membership-for-a-user
    ///
    /// Remove organization membership for a user
    /// In order to remove a user's membership with an organization, the authenticated user must be an organization owner.
    ///
    /// If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.
    DeleteOrgsorgMembershipsusername(String, String),
    /// * tags migrations
    /// * get `/orgs/{org}/migrations`
    /// * docs https://docs.github.com/rest/reference/migrations#list-organization-migrations
    ///
    /// List organization migrations
    /// Lists the most recent migrations.
    GetOrgsorgMigrations(String),
    /// * tags migrations
    /// * post `/orgs/{org}/migrations`
    /// * docs https://docs.github.com/rest/reference/migrations#start-an-organization-migration
    ///
    /// Start an organization migration
    /// Initiates the generation of a migration archive.
    PostOrgsorgMigrations(String),
    /// * tags migrations
    /// * get `/orgs/{org}/migrations/{migration_id}`
    /// * docs https://docs.github.com/rest/reference/migrations#get-an-organization-migration-status
    ///
    /// Get an organization migration status
    /// Fetches the status of a migration.
    ///
    /// The `state` of a migration can be one of the following values:
    ///
    /// *   `pending`, which means the migration hasn't started yet.
    /// *   `exporting`, which means the migration is in progress.
    /// *   `exported`, which means the migration finished successfully.
    /// *   `failed`, which means the migration failed.
    GetOrgsorgMigrationsmigrationId(String, String),
    /// * tags migrations
    /// * get `/orgs/{org}/migrations/{migration_id}/archive`
    /// * docs https://docs.github.com/rest/reference/migrations#download-an-organization-migration-archive
    ///
    /// Download an organization migration archive
    /// Fetches the URL to a migration archive.
    GetOrgsorgMigrationsmigrationIdArchive(String, String),
    /// * tags migrations
    /// * delete `/orgs/{org}/migrations/{migration_id}/archive`
    /// * docs https://docs.github.com/rest/reference/migrations#delete-an-organization-migration-archive
    ///
    /// Delete an organization migration archive
    /// Deletes a previous migration archive. Migration archives are automatically deleted after seven days.
    DeleteOrgsorgMigrationsmigrationIdArchive(String, String),
    /// * tags migrations
    /// * delete `/orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock`
    /// * docs https://docs.github.com/rest/reference/migrations#unlock-an-organization-repository
    ///
    /// Unlock an organization repository
    /// Unlocks a repository that was locked for migration. You should unlock each migrated repository and [delete them](https://docs.github.com/rest/reference/repos#delete-a-repository) when the migration is complete and you no longer need the source data.
    DeleteOrgsorgMigrationsmigrationIdReposrepoNameLock(String, String, String),
    /// * tags migrations
    /// * get `/orgs/{org}/migrations/{migration_id}/repositories`
    /// * docs https://docs.github.com/rest/reference/migrations#list-repositories-in-an-organization-migration
    ///
    /// List repositories in an organization migration
    /// List all the repositories for this organization migration.
    GetOrgsorgMigrationsmigrationIdRepositories(String, String),
    /// * tags orgs
    /// * get `/orgs/{org}/outside_collaborators`
    /// * docs https://docs.github.com/rest/reference/orgs#list-outside-collaborators-for-an-organization
    ///
    /// List outside collaborators for an organization
    /// List all users who are outside collaborators of an organization.
    GetOrgsorgOutsideCollaborators(String),
    /// * tags orgs
    /// * put `/orgs/{org}/outside_collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#convert-an-organization-member-to-outside-collaborator
    ///
    /// Convert an organization member to outside collaborator
    /// When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see "[Converting an organization member to an outside collaborator](https://help.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)".
    PutOrgsorgOutsideCollaboratorsusername(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/outside_collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#remove-outside-collaborator-from-an-organization
    ///
    /// Remove outside collaborator from an organization
    /// Removing a user from this list will remove them from all the organization's repositories.
    DeleteOrgsorgOutsideCollaboratorsusername(String, String),
    /// * tags packages
    /// * get `/orgs/{org}/packages`
    /// * docs https://docs.github.com/rest/reference/packages#list-packages-for-an-organization
    ///
    /// List packages for an organization
    /// Lists all packages in an organization readable by the user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetOrgsorgPackages(String),
    /// * tags packages
    /// * get `/orgs/{org}/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-for-an-organization
    ///
    /// Get a package for an organization
    /// Gets a specific package in an organization.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetOrgsorgPackagespackageTypepackageName(String, String, String),
    /// * tags packages
    /// * delete `/orgs/{org}/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-for-an-organization
    ///
    /// Delete a package for an organization
    /// Deletes an entire package in an organization. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
    DeleteOrgsorgPackagespackageTypepackageName(String, String, String),
    /// * tags packages
    /// * post `/orgs/{org}/packages/{package_type}/{package_name}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-for-an-organization
    ///
    /// Restore a package for an organization
    /// Restores an entire package in an organization.
    ///
    /// You can restore a deleted package under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
    PostOrgsorgPackagespackageTypepackageNameRestore(String, String, String),
    /// * tags packages
    /// * get `/orgs/{org}/packages/{package_type}/{package_name}/versions`
    /// * docs https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-an-organization
    ///
    /// Get all package versions for a package owned by an organization
    /// Returns all package versions for a package owned by an organization.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetOrgsorgPackagespackageTypepackageNameVersions(String, String, String),
    /// * tags packages
    /// * get `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-version-for-an-organization
    ///
    /// Get a package version for an organization
    /// Gets a specific package version in an organization.
    ///
    /// You must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(
        String,
        String,
        String,
        String,
    ),
    /// * tags packages
    /// * delete `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-version-for-an-organization
    ///
    /// Delete package version for an organization
    /// Deletes a specific package version in an organization. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
    DeleteOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(
        String,
        String,
        String,
        String,
    ),
    /// * tags packages
    /// * post `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-version-for-an-organization
    ///
    /// Restore package version for an organization
    /// Restores a specific package version in an organization.
    ///
    /// You can restore a deleted package under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
    PostOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdRestore(
        String,
        String,
        String,
        String,
    ),
    /// * tags projects
    /// * get `/orgs/{org}/projects`
    /// * docs https://docs.github.com/rest/reference/projects#list-organization-projects
    ///
    /// List organization projects
    /// Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    GetOrgsorgProjects(String),
    /// * tags projects
    /// * post `/orgs/{org}/projects`
    /// * docs https://docs.github.com/rest/reference/projects#create-an-organization-project
    ///
    /// Create an organization project
    /// Creates an organization project board. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    PostOrgsorgProjects(String),
    /// * tags orgs
    /// * get `/orgs/{org}/public_members`
    /// * docs https://docs.github.com/rest/reference/orgs#list-public-organization-members
    ///
    /// List public organization members
    /// Members of an organization can choose to have their membership publicized or not.
    GetOrgsorgPublicMembers(String),
    /// * tags orgs
    /// * get `/orgs/{org}/public_members/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#check-public-organization-membership-for-a-user
    ///
    /// Check public organization membership for a user
    ///
    GetOrgsorgPublicMembersusername(String, String),
    /// * tags orgs
    /// * put `/orgs/{org}/public_members/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#set-public-organization-membership-for-the-authenticated-user
    ///
    /// Set public organization membership for the authenticated user
    /// The user can publicize their own membership. (A user cannot publicize the membership for another user.)
    ///
    /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutOrgsorgPublicMembersusername(String, String),
    /// * tags orgs
    /// * delete `/orgs/{org}/public_members/{username}`
    /// * docs https://docs.github.com/rest/reference/orgs#remove-public-organization-membership-for-the-authenticated-user
    ///
    /// Remove public organization membership for the authenticated user
    ///
    DeleteOrgsorgPublicMembersusername(String, String),
    /// * tags repos
    /// * get `/orgs/{org}/repos`
    /// * docs https://docs.github.com/rest/reference/repos#list-organization-repositories
    ///
    /// List organization repositories
    /// Lists repositories for the specified organization.
    GetOrgsorgRepos(String),
    /// * tags repos
    /// * post `/orgs/{org}/repos`
    /// * docs https://docs.github.com/rest/reference/repos#create-an-organization-repository
    ///
    /// Create an organization repository
    /// Creates a new repository in the specified organization. The authenticated user must be a member of the organization.
    ///
    /// **OAuth scope requirements**
    ///
    /// When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
    ///
    /// *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
    /// *   `repo` scope to create a private repository
    PostOrgsorgRepos(String),
    /// * tags secret-scanning
    /// * get `/orgs/{org}/secret-scanning/alerts`
    /// * docs https://docs.github.com/rest/reference/secret-scanning#list-secret-scanning-alerts-for-an-organization
    ///
    /// List secret scanning alerts for an organization
    /// Lists secret scanning alerts for eligible repositories in an organization, from newest to oldest.
    /// To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
    GetOrgsorgSecretScanningAlerts(String),
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/actions`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-organization
    ///
    /// Get GitHub Actions billing for an organization
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    GetOrgsorgSettingsBillingActions(String),
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/advanced-security`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-advanced-security-active-committers-for-an-organization
    ///
    /// Get GitHub Advanced Security active committers for an organization
    /// Gets the GitHub Advanced Security active committers for an organization per repository.
    /// Each distinct user login across all repositories is counted as a single Advanced Security seat, so the total_advanced_security_committers is not the sum of advanced_security_committers for each repository.
    /// If this organization defers to an enterprise for billing, the total_advanced_security_committers returned from the organization API may include some users that are in more than one organization, so they will only consume a single Advanced Security seat at the enterprise level.
    GetOrgsorgSettingsBillingAdvancedSecurity(String),
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/packages`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-organization
    ///
    /// Get GitHub Packages billing for an organization
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    GetOrgsorgSettingsBillingPackages(String),
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/shared-storage`
    /// * docs https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-organization
    ///
    /// Get shared storage billing for an organization
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    GetOrgsorgSettingsBillingSharedStorage(String),
    /// * tags teams
    /// * get `/orgs/{org}/team-sync/groups`
    /// * docs https://docs.github.com/rest/reference/teams#list-idp-groups-for-an-organization
    ///
    /// List IdP groups for an organization
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups available in an organization. You can limit your page results using the `per_page` parameter. GitHub generates a url-encoded `page` token using a cursor value for where the next page begins. For more information on cursor pagination, see "[Offset and Cursor Pagination explained](https://dev.to/jackmarchant/offset-and-cursor-pagination-explained-b89)."
    GetOrgsorgTeamSyncGroups(String),
    /// * tags teams
    /// * get `/orgs/{org}/teams`
    /// * docs https://docs.github.com/rest/reference/teams#list-teams
    ///
    /// List teams
    /// Lists all teams in an organization that are visible to the authenticated user.
    GetOrgsorgTeams(String),
    /// * tags teams
    /// * post `/orgs/{org}/teams`
    /// * docs https://docs.github.com/rest/reference/teams#create-a-team
    ///
    /// Create a team
    /// To create a team, the authenticated user must be a member or owner of `{org}`. By default, organization members can create teams. Organization owners can limit team creation to organization owners. For more information, see "[Setting team creation permissions](https://help.github.com/en/articles/setting-team-creation-permissions-in-your-organization)."
    ///
    /// When you create a new team, you automatically become a team maintainer without explicitly adding yourself to the optional array of `maintainers`. For more information, see "[About teams](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/about-teams)".
    PostOrgsorgTeams(String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}`
    /// * docs https://docs.github.com/rest/reference/teams#get-a-team-by-name
    ///
    /// Get a team by name
    /// Gets a team using the team's `slug`. GitHub generates the `slug` from the team `name`.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.
    GetOrgsorgTeamsteamSlug(String, String),
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}`
    /// * docs https://docs.github.com/rest/reference/teams#update-a-team
    ///
    /// Update a team
    /// To edit a team, the authenticated user must either be an organization owner or a team maintainer.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}`.
    PatchOrgsorgTeamsteamSlug(String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}`
    /// * docs https://docs.github.com/rest/reference/teams#delete-a-team
    ///
    /// Delete a team
    /// To delete a team, the authenticated user must be an organization owner or team maintainer.
    ///
    /// If you are an organization owner, deleting a parent team will delete all of its child teams as well.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}`.
    DeleteOrgsorgTeamsteamSlug(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions`
    /// * docs https://docs.github.com/rest/reference/teams#list-discussions
    ///
    /// List discussions
    /// List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions`.
    GetOrgsorgTeamsteamSlugDiscussions(String, String),
    /// * tags teams
    /// * post `/orgs/{org}/teams/{team_slug}/discussions`
    /// * docs https://docs.github.com/rest/reference/teams#create-a-discussion
    ///
    /// Create a discussion
    /// Creates a new discussion post on a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions`.
    PostOrgsorgTeamsteamSlugDiscussions(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#get-a-discussion
    ///
    /// Get a discussion
    /// Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
    GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(String, String, String),
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#update-a-discussion
    ///
    /// Update a discussion
    /// Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
    PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(String, String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#delete-a-discussion
    ///
    /// Delete a discussion
    /// Delete a discussion from a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
    DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(String, String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments`
    /// * docs https://docs.github.com/rest/reference/teams#list-discussion-comments
    ///
    /// List discussion comments
    /// List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
    GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(String, String, String),
    /// * tags teams
    /// * post `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments`
    /// * docs https://docs.github.com/rest/reference/teams#create-a-discussion-comment
    ///
    /// Create a discussion comment
    /// Creates a new comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.
    PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(String, String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#get-a-discussion-comment
    ///
    /// Get a discussion comment
    /// Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
    GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
        String,
        String,
        String,
        String,
    ),
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#update-a-discussion-comment
    ///
    /// Update a discussion comment
    /// Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
    PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
        String,
        String,
        String,
        String,
    ),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#delete-a-discussion-comment
    ///
    /// Delete a discussion comment
    /// Deletes a comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
    DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
        String,
        String,
        String,
        String,
    ),
    /// * tags reactions
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment
    ///
    /// List reactions for a team discussion comment
    /// List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments/). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
    GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(
        String,
        String,
        String,
        String,
    ),
    /// * tags reactions
    /// * post `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion-comment
    ///
    /// Create reaction for a team discussion comment
    /// Create a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
    PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(
        String,
        String,
        String,
        String,
    ),
    /// * tags reactions
    /// * delete `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-team-discussion-comment-reaction
    ///
    /// Delete team discussion comment reaction
    /// **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions/:reaction_id`.
    ///
    /// Delete a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsreactionId(
        String,
        String,
        String,
        String,
        String,
    ),
    /// * tags reactions
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion
    ///
    /// List reactions for a team discussion
    /// List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
    GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(String, String, String),
    /// * tags reactions
    /// * post `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion
    ///
    /// Create reaction for a team discussion
    /// Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
    PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(String, String, String),
    /// * tags reactions
    /// * delete `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-team-discussion-reaction
    ///
    /// Delete team discussion reaction
    /// **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions/:reaction_id`.
    ///
    /// Delete a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsreactionId(
        String,
        String,
        String,
        String,
    ),
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/external-groups`
    /// * docs https://docs.github.com/rest/reference/teams#link-external-idp-group-team-connection
    ///
    /// Update the connection between an external group and a team
    /// Creates a connection between a team and an external group.  Only one external group can be linked to a team.
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    PatchOrgsorgTeamsteamSlugExternalGroups(String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/external-groups`
    /// * docs https://docs.github.com/rest/reference/teams#unlink-external-idp-group-team-connection
    ///
    /// Remove the connection between an external group and a team
    /// Deletes a connection between a team and an external group.
    ///
    /// You can manage team membership with your IdP using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    DeleteOrgsorgTeamsteamSlugExternalGroups(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/invitations`
    /// * docs https://docs.github.com/rest/reference/teams#list-pending-team-invitations
    ///
    /// List pending team invitations
    /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.
    GetOrgsorgTeamsteamSlugInvitations(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/members`
    /// * docs https://docs.github.com/rest/reference/teams#list-team-members
    ///
    /// List team members
    /// Team members will include the members of child teams.
    ///
    /// To list members in a team, the team must be visible to the authenticated user.
    GetOrgsorgTeamsteamSlugMembers(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user
    ///
    /// Get team membership for a user
    /// Team members will include the members of child teams.
    ///
    /// To get a user's membership with a team, the team must be visible to the authenticated user.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.
    ///
    /// **Note:**
    /// The response contains the `state` of the membership and the member's `role`.
    ///
    /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
    GetOrgsorgTeamsteamSlugMembershipsusername(String, String, String),
    /// * tags teams
    /// * put `/orgs/{org}/teams/{team_slug}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user
    ///
    /// Add or update team membership for a user
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the "pending" state until the person accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team.
    ///
    /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.
    PutOrgsorgTeamsteamSlugMembershipsusername(String, String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user
    ///
    /// Remove team membership for a user
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/memberships/{username}`.
    DeleteOrgsorgTeamsteamSlugMembershipsusername(String, String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/projects`
    /// * docs https://docs.github.com/rest/reference/teams#list-team-projects
    ///
    /// List team projects
    /// Lists the organization projects for a team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects`.
    GetOrgsorgTeamsteamSlugProjects(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project
    ///
    /// Check team permissions for a project
    /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
    GetOrgsorgTeamsteamSlugProjectsprojectId(String, String, String),
    /// * tags teams
    /// * put `/orgs/{org}/teams/{team_slug}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams#add-or-update-team-project-permissions
    ///
    /// Add or update team project permissions
    /// Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
    PutOrgsorgTeamsteamSlugProjectsprojectId(String, String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams#remove-a-project-from-a-team
    ///
    /// Remove a project from a team
    /// Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. This endpoint removes the project from the team, but does not delete the project.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
    DeleteOrgsorgTeamsteamSlugProjectsprojectId(String, String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/repos`
    /// * docs https://docs.github.com/rest/reference/teams#list-team-repositories
    ///
    /// List team repositories
    /// Lists a team's repositories visible to the authenticated user.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.
    GetOrgsorgTeamsteamSlugRepos(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository
    ///
    /// Check team permissions for a repository
    /// Checks whether a team has `admin`, `push`, `maintain`, `triage`, or `pull` permission for a repository. Repositories inherited through a parent team will also be checked.
    ///
    /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `application/vnd.github.v3.repository+json` accept header.
    ///
    /// If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
    GetOrgsorgTeamsteamSlugReposownerrepo(String, String, String, String),
    /// * tags teams
    /// * put `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#add-or-update-team-repository-permissions
    ///
    /// Add or update team repository permissions
    /// To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
    ///
    /// For more information about the permission levels, see "[Repository permission levels for an organization](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)".
    PutOrgsorgTeamsteamSlugReposownerrepo(String, String, String, String),
    /// * tags teams
    /// * delete `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#remove-a-repository-from-a-team
    ///
    /// Remove a repository from a team
    /// If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. This does not delete the repository, it just removes it from the team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
    DeleteOrgsorgTeamsteamSlugReposownerrepo(String, String, String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings`
    /// * docs https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team
    ///
    /// List IdP groups for a team
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups connected to a team on GitHub.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
    GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(String, String),
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings`
    /// * docs https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections
    ///
    /// Create or update IdP group connections
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
    PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(String, String),
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/teams`
    /// * docs https://docs.github.com/rest/reference/teams#list-child-teams
    ///
    /// List child teams
    /// Lists the child teams of the team specified by `{team_slug}`.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.
    GetOrgsorgTeamsteamSlugTeams(String, String),
    /// * tags projects
    /// * get `/projects/columns/cards/{card_id}`
    /// * docs https://docs.github.com/rest/reference/projects#get-a-project-card
    ///
    /// Get a project card
    ///
    GetProjectsColumnsCardscardId(String),
    /// * tags projects
    /// * patch `/projects/columns/cards/{card_id}`
    /// * docs https://docs.github.com/rest/reference/projects#update-a-project-card
    ///
    /// Update an existing project card
    ///
    PatchProjectsColumnsCardscardId(String),
    /// * tags projects
    /// * delete `/projects/columns/cards/{card_id}`
    /// * docs https://docs.github.com/rest/reference/projects#delete-a-project-card
    ///
    /// Delete a project card
    ///
    DeleteProjectsColumnsCardscardId(String),
    /// * tags projects
    /// * post `/projects/columns/cards/{card_id}/moves`
    /// * docs https://docs.github.com/rest/reference/projects#move-a-project-card
    ///
    /// Move a project card
    ///
    PostProjectsColumnsCardscardIdMoves(String),
    /// * tags projects
    /// * get `/projects/columns/{column_id}`
    /// * docs https://docs.github.com/rest/reference/projects#get-a-project-column
    ///
    /// Get a project column
    ///
    GetProjectsColumnscolumnId(String),
    /// * tags projects
    /// * patch `/projects/columns/{column_id}`
    /// * docs https://docs.github.com/rest/reference/projects#update-a-project-column
    ///
    /// Update an existing project column
    ///
    PatchProjectsColumnscolumnId(String),
    /// * tags projects
    /// * delete `/projects/columns/{column_id}`
    /// * docs https://docs.github.com/rest/reference/projects#delete-a-project-column
    ///
    /// Delete a project column
    ///
    DeleteProjectsColumnscolumnId(String),
    /// * tags projects
    /// * get `/projects/columns/{column_id}/cards`
    /// * docs https://docs.github.com/rest/reference/projects#list-project-cards
    ///
    /// List project cards
    ///
    GetProjectsColumnscolumnIdCards(String),
    /// * tags projects
    /// * post `/projects/columns/{column_id}/cards`
    /// * docs https://docs.github.com/rest/reference/projects#create-a-project-card
    ///
    /// Create a project card
    ///
    PostProjectsColumnscolumnIdCards(String),
    /// * tags projects
    /// * post `/projects/columns/{column_id}/moves`
    /// * docs https://docs.github.com/rest/reference/projects#move-a-project-column
    ///
    /// Move a project column
    ///
    PostProjectsColumnscolumnIdMoves(String),
    /// * tags projects
    /// * get `/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/projects#get-a-project
    ///
    /// Get a project
    /// Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    GetProjectsprojectId(String),
    /// * tags projects
    /// * patch `/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/projects#update-a-project
    ///
    /// Update a project
    /// Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    PatchProjectsprojectId(String),
    /// * tags projects
    /// * delete `/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/projects#delete-a-project
    ///
    /// Delete a project
    /// Deletes a project board. Returns a `404 Not Found` status if projects are disabled.
    DeleteProjectsprojectId(String),
    /// * tags projects
    /// * get `/projects/{project_id}/collaborators`
    /// * docs https://docs.github.com/rest/reference/projects#list-project-collaborators
    ///
    /// List project collaborators
    /// Lists the collaborators for an organization project. For a project, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. You must be an organization owner or a project `admin` to list collaborators.
    GetProjectsprojectIdCollaborators(String),
    /// * tags projects
    /// * put `/projects/{project_id}/collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/projects#add-project-collaborator
    ///
    /// Add project collaborator
    /// Adds a collaborator to an organization project and sets their permission level. You must be an organization owner or a project `admin` to add a collaborator.
    PutProjectsprojectIdCollaboratorsusername(String, String),
    /// * tags projects
    /// * delete `/projects/{project_id}/collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/projects#remove-project-collaborator
    ///
    /// Remove user as a collaborator
    /// Removes a collaborator from an organization project. You must be an organization owner or a project `admin` to remove a collaborator.
    DeleteProjectsprojectIdCollaboratorsusername(String, String),
    /// * tags projects
    /// * get `/projects/{project_id}/collaborators/{username}/permission`
    /// * docs https://docs.github.com/rest/reference/projects#get-project-permission-for-a-user
    ///
    /// Get project permission for a user
    /// Returns the collaborator's permission level for an organization project. Possible values for the `permission` key: `admin`, `write`, `read`, `none`. You must be an organization owner or a project `admin` to review a user's permission level.
    GetProjectsprojectIdCollaboratorsusernamePermission(String, String),
    /// * tags projects
    /// * get `/projects/{project_id}/columns`
    /// * docs https://docs.github.com/rest/reference/projects#list-project-columns
    ///
    /// List project columns
    ///
    GetProjectsprojectIdColumns(String),
    /// * tags projects
    /// * post `/projects/{project_id}/columns`
    /// * docs https://docs.github.com/rest/reference/projects#create-a-project-column
    ///
    /// Create a project column
    ///
    PostProjectsprojectIdColumns(String),
    /// * tags rate-limit
    /// * get `/rate_limit`
    /// * docs https://docs.github.com/rest/reference/rate-limit#get-rate-limit-status-for-the-authenticated-user
    ///
    /// Get rate limit status for the authenticated user
    /// **Note:** Accessing this endpoint does not count against your REST API rate limit.
    ///
    /// **Note:** The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    GetRateLimit(),
    /// * tags reactions
    /// * delete `/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions/#delete-a-reaction-legacy
    ///
    /// Delete a reaction (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Reactions API. We recommend migrating your existing code to use the new delete reactions endpoints. For more information, see this [blog post](https://developer.github.com/changes/2020-02-26-new-delete-reactions-endpoints/).
    ///
    /// OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), when deleting a [team discussion](https://docs.github.com/rest/reference/teams#discussions) or [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments).
    DeleteReactionsreactionId(String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-repository
    ///
    /// Get a repository
    /// The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.
    GetReposownerrepo(String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/repos/#update-a-repository
    ///
    /// Update a repository
    /// **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/reference/repos#replace-all-repository-topics) endpoint.
    PatchReposownerrepo(String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-repository
    ///
    /// Delete a repository
    /// Deleting a repository requires admin access. If OAuth is used, the `delete_repo` scope is required.
    ///
    /// If an organization owner has configured the organization to prevent members from deleting organization-owned
    /// repositories, you will get a `403 Forbidden` response.
    DeleteReposownerrepo(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/artifacts`
    /// * docs https://docs.github.com/rest/reference/actions#list-artifacts-for-a-repository
    ///
    /// List artifacts for a repository
    /// Lists all artifacts for a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsArtifacts(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-an-artifact
    ///
    /// Get an artifact
    /// Gets a specific artifact for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsArtifactsartifactId(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-an-artifact
    ///
    /// Delete an artifact
    /// Deletes an artifact for a workflow run. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    DeleteReposownerrepoActionsArtifactsartifactId(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}`
    /// * docs https://docs.github.com/rest/reference/actions#download-an-artifact
    ///
    /// Download an artifact
    /// Gets a redirect URL to download an archive for a repository. This URL expires after 1 minute. Look for `Location:` in
    /// the response header to find the URL for the download. The `:archive_format` must be `zip`. Anyone with read access to
    /// the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
    /// GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsArtifactsartifactIdarchiveFormat(String, String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/jobs/{job_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-job-for-a-workflow-run
    ///
    /// Get a job for a workflow run
    /// Gets a specific job in a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsJobsjobId(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/jobs/{job_id}/logs`
    /// * docs https://docs.github.com/rest/reference/actions#download-job-logs-for-a-workflow-run
    ///
    /// Download job logs for a workflow run
    /// Gets a redirect URL to download a plain text file of logs for a workflow job. This link expires after 1 minute. Look
    /// for `Location:` in the response header to find the URL for the download. Anyone with read access to the repository can
    /// use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must
    /// have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsJobsjobIdLogs(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-a-repository
    ///
    /// Get GitHub Actions permissions for a repository
    /// Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions allowed to run in the repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    GetReposownerrepoActionsPermissions(String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/permissions`
    /// * docs https://docs.github.com/rest/reference/actions#set-github-actions-permissions-for-a-repository
    ///
    /// Set GitHub Actions permissions for a repository
    /// Sets the GitHub Actions permissions policy for enabling GitHub Actions and allowed actions in the repository.
    ///
    /// If the repository belongs to an organization or enterprise that has set restrictive permissions at the organization or enterprise levels, such as `allowed_actions` to `selected` actions, then you cannot override them for the repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    PutReposownerrepoActionsPermissions(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/actions#get-allowed-actions-for-a-repository
    ///
    /// Get allowed actions for a repository
    /// Gets the settings for selected actions that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    GetReposownerrepoActionsPermissionsSelectedActions(String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/permissions/selected-actions`
    /// * docs https://docs.github.com/rest/reference/actions#set-allowed-actions-for-a-repository
    ///
    /// Set allowed actions for a repository
    /// Sets the actions that are allowed in a repository. To use this endpoint, the repository permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
    ///
    /// If the repository belongs to an organization or enterprise that has `selected` actions set at the organization or enterprise levels, then you cannot override any of the allowed actions settings.
    ///
    /// To use the `patterns_allowed` setting for private repositories, the repository must belong to an enterprise. If the repository does not belong to an enterprise, then the `patterns_allowed` setting only applies to public repositories.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    PutReposownerrepoActionsPermissionsSelectedActions(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners`
    /// * docs https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-a-repository
    ///
    /// List self-hosted runners for a repository
    /// Lists all self-hosted runners configured in a repository. You must authenticate using an access token with the `repo` scope to use this endpoint.
    GetReposownerrepoActionsRunners(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners/downloads`
    /// * docs https://docs.github.com/rest/reference/actions#list-runner-applications-for-a-repository
    ///
    /// List runner applications for a repository
    /// Lists binaries for the runner application that you can download and run.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint.
    GetReposownerrepoActionsRunnersDownloads(String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runners/registration-token`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-registration-token-for-a-repository
    ///
    /// Create a registration token for a repository
    /// Returns a token that you can pass to the `config` script. The token expires after one hour. You must authenticate
    /// using an access token with the `repo` scope to use this endpoint.
    ///
    /// #### Example using registration token
    ///  
    /// Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
    ///
    /// ```
    /// ./config.sh --url https://github.com/octo-org/octo-repo-artifacts --token TOKEN
    /// ```
    PostReposownerrepoActionsRunnersRegistrationToken(String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runners/remove-token`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-remove-token-for-a-repository
    ///
    /// Create a remove token for a repository
    /// Returns a token that you can pass to remove a self-hosted runner from a repository. The token expires after one hour.
    /// You must authenticate using an access token with the `repo` scope to use this endpoint.
    ///
    /// #### Example using remove token
    ///  
    /// To remove your self-hosted runner from a repository, replace TOKEN with the remove token provided by this endpoint.
    ///
    /// ```
    /// ./config.sh remove --token TOKEN
    /// ```
    PostReposownerrepoActionsRunnersRemoveToken(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-a-repository
    ///
    /// Get a self-hosted runner for a repository
    /// Gets a specific self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    GetReposownerrepoActionsRunnersrunnerId(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runners/{runner_id}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-from-a-repository
    ///
    /// Delete a self-hosted runner from a repository
    /// Forces the removal of a self-hosted runner from a repository. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
    ///
    /// You must authenticate using an access token with the `repo`
    /// scope to use this endpoint.
    DeleteReposownerrepoActionsRunnersrunnerId(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#list-labels-for-a-self-hosted-runner-for-a-repository
    ///
    /// List labels for a self-hosted runner for a repository
    /// Lists all labels for a self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    GetReposownerrepoActionsRunnersrunnerIdLabels(String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#add-custom-labels-to-a-self-hosted-runner-for-a-repository
    ///
    /// Add custom labels to a self-hosted runner for a repository
    /// Add custom labels to a self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    PostReposownerrepoActionsRunnersrunnerIdLabels(String, String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#set-custom-labels-for-a-self-hosted-runner-for-a-repository
    ///
    /// Set custom labels for a self-hosted runner for a repository
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    PutReposownerrepoActionsRunnersrunnerIdLabels(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs https://docs.github.com/rest/reference/actions#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository
    ///
    /// Remove all custom labels from a self-hosted runner for a repository
    /// Remove all custom labels from a self-hosted runner configured in a
    /// repository. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    DeleteReposownerrepoActionsRunnersrunnerIdLabels(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/actions#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository
    ///
    /// Remove a custom label from a self-hosted runner for a repository
    /// Remove a custom label from a self-hosted runner configured
    /// in a repository. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(String, String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs`
    /// * docs https://docs.github.com/rest/reference/actions#list-workflow-runs-for-a-repository
    ///
    /// List workflow runs for a repository
    /// Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRuns(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-workflow-run
    ///
    /// Get a workflow run
    /// Gets a specific workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunId(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runs/{run_id}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-a-workflow-run
    ///
    /// Delete a workflow run
    /// Delete a specific workflow run. Anyone with write access to the repository can use this endpoint. If the repository is
    /// private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:write` permission to use
    /// this endpoint.
    DeleteReposownerrepoActionsRunsrunId(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/approvals`
    /// * docs https://docs.github.com/rest/reference/actions#get-the-review-history-for-a-workflow-run
    ///
    /// Get the review history for a workflow run
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdApprovals(String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runs/{run_id}/approve`
    /// * docs https://docs.github.com/rest/reference/actions#approve-a-workflow-run-for-a-fork-pull-request
    ///
    /// Approve a workflow run for a fork pull request
    /// Approves a workflow run for a pull request from a public fork of a first time contributor. For more information, see ["Approving workflow runs from public forks](https://docs.github.com/actions/managing-workflow-runs/approving-workflow-runs-from-public-forks)."
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    PostReposownerrepoActionsRunsrunIdApprove(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts`
    /// * docs https://docs.github.com/rest/reference/actions#list-workflow-run-artifacts
    ///
    /// List workflow run artifacts
    /// Lists artifacts for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdArtifacts(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-workflow-run-attempt
    ///
    /// Get a workflow run attempt
    /// Gets a specific workflow run attempt. Anyone with read access to the repository
    /// can use this endpoint. If the repository is private you must use an access token
    /// with the `repo` scope. GitHub Apps must have the `actions:read` permission to
    /// use this endpoint.
    GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(String, String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs`
    /// * docs https://docs.github.com/rest/reference/actions#list-jobs-for-a-workflow-run-attempt
    ///
    /// List jobs for a workflow run attempt
    /// Lists jobs for a specific workflow run attempt. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(String, String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs`
    /// * docs https://docs.github.com/rest/reference/actions#download-workflow-run-attempt-logs
    ///
    /// Download workflow run attempt logs
    /// Gets a redirect URL to download an archive of log files for a specific workflow run attempt. This link expires after
    /// 1 minute. Look for `Location:` in the response header to find the URL for the download. Anyone with read access to
    /// the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
    /// GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberLogs(String, String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runs/{run_id}/cancel`
    /// * docs https://docs.github.com/rest/reference/actions#cancel-a-workflow-run
    ///
    /// Cancel a workflow run
    /// Cancels a workflow run using its `id`. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    PostReposownerrepoActionsRunsrunIdCancel(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/jobs`
    /// * docs https://docs.github.com/rest/reference/actions#list-jobs-for-a-workflow-run
    ///
    /// List jobs for a workflow run
    /// Lists jobs for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    GetReposownerrepoActionsRunsrunIdJobs(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/logs`
    /// * docs https://docs.github.com/rest/reference/actions#download-workflow-run-logs
    ///
    /// Download workflow run logs
    /// Gets a redirect URL to download an archive of log files for a workflow run. This link expires after 1 minute. Look for
    /// `Location:` in the response header to find the URL for the download. Anyone with read access to the repository can use
    /// this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have
    /// the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdLogs(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runs/{run_id}/logs`
    /// * docs https://docs.github.com/rest/reference/actions#delete-workflow-run-logs
    ///
    /// Delete workflow run logs
    /// Deletes all logs for a workflow run. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    DeleteReposownerrepoActionsRunsrunIdLogs(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments`
    /// * docs https://docs.github.com/rest/reference/actions#get-pending-deployments-for-a-workflow-run
    ///
    /// Get pending deployments for a workflow run
    /// Get all deployment environments for a workflow run that are waiting for protection rules to pass.
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdPendingDeployments(String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments`
    /// * docs https://docs.github.com/rest/reference/actions#review-pending-deployments-for-a-workflow-run
    ///
    /// Review pending deployments for a workflow run
    /// Approve or reject pending deployments that are waiting on approval by a required reviewer.
    ///
    /// Anyone with read access to the repository contents and deployments can use this endpoint.
    PostReposownerrepoActionsRunsrunIdPendingDeployments(String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runs/{run_id}/rerun`
    /// * docs https://docs.github.com/rest/reference/actions#re-run-a-workflow
    ///
    /// Re-run a workflow
    /// Re-runs your workflow run using its `id`. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    PostReposownerrepoActionsRunsrunIdRerun(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/timing`
    /// * docs https://docs.github.com/rest/reference/actions#get-workflow-run-usage
    ///
    /// Get workflow run usage
    /// Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsRunsrunIdTiming(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets`
    /// * docs https://docs.github.com/rest/reference/actions#list-repository-secrets
    ///
    /// List repository secrets
    /// Lists all secrets available in a repository without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetReposownerrepoActionsSecrets(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets/public-key`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-repository-public-key
    ///
    /// Get a repository public key
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetReposownerrepoActionsSecretsPublicKey(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-repository-secret
    ///
    /// Get a repository secret
    /// Gets a single repository secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetReposownerrepoActionsSecretssecretName(String, String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#create-or-update-a-repository-secret
    ///
    /// Create or update a repository secret
    /// Creates or updates a repository secret with an encrypted value. Encrypt your secret using
    /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
    /// token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use
    /// this endpoint.
    ///
    /// #### Example encrypting a secret using Node.js
    ///
    /// Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
    ///
    /// ```
    /// const sodium = require('tweetsodium');
    ///
    /// const key = "base64-encoded-public-key";
    /// const value = "plain-text-secret";
    ///
    /// // Convert the message and key to Uint8Array's (Buffer implements that interface)
    /// const messageBytes = Buffer.from(value);
    /// const keyBytes = Buffer.from(key, 'base64');
    ///
    /// // Encrypt using LibSodium.
    /// const encryptedBytes = sodium.seal(messageBytes, keyBytes);
    ///
    /// // Base64 the encrypted secret
    /// const encrypted = Buffer.from(encryptedBytes).toString('base64');
    ///
    /// console.log(encrypted);
    /// ```
    ///
    ///
    /// #### Example encrypting a secret using Python
    ///
    /// Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
    ///
    /// ```
    /// from base64 import b64encode
    /// from nacl import encoding, public
    ///
    /// def encrypt(public_key: str, secret_value: str) -> str:
    ///   """Encrypt a Unicode string using the public key."""
    ///   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
    ///   sealed_box = public.SealedBox(public_key)
    ///   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
    ///   return b64encode(encrypted).decode("utf-8")
    /// ```
    ///
    /// #### Example encrypting a secret using C#
    ///
    /// Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
    ///
    /// ```
    /// var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
    /// var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
    ///
    /// var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
    ///
    /// Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
    /// ```
    ///
    /// #### Example encrypting a secret using Ruby
    ///
    /// Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
    ///
    /// ```ruby
    /// require "rbnacl"
    /// require "base64"
    ///
    /// key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
    /// public_key = RbNaCl::PublicKey.new(key)
    ///
    /// box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
    /// encrypted_secret = box.encrypt("my_secret")
    ///
    /// # Print the base64 encoded secret
    /// puts Base64.strict_encode64(encrypted_secret)
    /// ```
    PutReposownerrepoActionsSecretssecretName(String, String, String),
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-a-repository-secret
    ///
    /// Delete a repository secret
    /// Deletes a secret in a repository using the secret name. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    DeleteReposownerrepoActionsSecretssecretName(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows`
    /// * docs https://docs.github.com/rest/reference/actions#list-repository-workflows
    ///
    /// List repository workflows
    /// Lists the workflows in a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsWorkflows(String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}`
    /// * docs https://docs.github.com/rest/reference/actions#get-a-workflow
    ///
    /// Get a workflow
    /// Gets a specific workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsWorkflowsworkflowId(String, String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable`
    /// * docs https://docs.github.com/rest/reference/actions#disable-a-workflow
    ///
    /// Disable a workflow
    /// Disables a workflow and sets the `state` of the workflow to `disabled_manually`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    PutReposownerrepoActionsWorkflowsworkflowIdDisable(String, String, String),
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches`
    /// * docs https://docs.github.com/rest/reference/actions#create-a-workflow-dispatch-event
    ///
    /// Create a workflow dispatch event
    /// You can use this endpoint to manually trigger a GitHub Actions workflow run. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
    ///
    /// You must configure your GitHub Actions workflow to run when the [`workflow_dispatch` webhook](/developers/webhooks-and-events/webhook-events-and-payloads#workflow_dispatch) event occurs. The `inputs` are configured in the workflow file. For more information about how to configure the `workflow_dispatch` event in the workflow file, see "[Events that trigger workflows](/actions/reference/events-that-trigger-workflows#workflow_dispatch)."
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint. For more information, see "[Creating a personal access token for the command line](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line)."
    PostReposownerrepoActionsWorkflowsworkflowIdDispatches(String, String, String),
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable`
    /// * docs https://docs.github.com/rest/reference/actions#enable-a-workflow
    ///
    /// Enable a workflow
    /// Enables a workflow and sets the `state` of the workflow to `active`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
    PutReposownerrepoActionsWorkflowsworkflowIdEnable(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs`
    /// * docs https://docs.github.com/rest/reference/actions#list-workflow-runs
    ///
    /// List workflow runs
    /// List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
    GetReposownerrepoActionsWorkflowsworkflowIdRuns(String, String, String),
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing`
    /// * docs https://docs.github.com/rest/reference/actions#get-workflow-usage
    ///
    /// Get workflow usage
    /// Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoActionsWorkflowsworkflowIdTiming(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/assignees`
    /// * docs https://docs.github.com/rest/reference/issues#list-assignees
    ///
    /// List assignees
    /// Lists the [available assignees](https://help.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
    GetReposownerrepoAssignees(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/assignees/{assignee}`
    /// * docs https://docs.github.com/rest/reference/issues#check-if-a-user-can-be-assigned
    ///
    /// Check if a user can be assigned
    /// Checks if a user has permission to be assigned to an issue in this repository.
    ///
    /// If the `assignee` can be assigned to issues in the repository, a `204` header with no content is returned.
    ///
    /// Otherwise a `404` status code is returned.
    GetReposownerrepoAssigneesassignee(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/autolinks`
    /// * docs https://docs.github.com/v3/repos#list-autolinks
    ///
    /// List all autolinks of a repository
    /// This returns a list of autolinks configured for the given repository.
    ///
    /// Information about autolinks are only available to repository administrators.
    GetReposownerrepoAutolinks(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/autolinks`
    /// * docs https://docs.github.com/v3/repos#create-an-autolink
    ///
    /// Create an autolink reference for a repository
    /// Users with admin access to the repository can create an autolink.
    PostReposownerrepoAutolinks(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/autolinks/{autolink_id}`
    /// * docs https://docs.github.com/v3/repos#get-autolink
    ///
    /// Get an autolink reference of a repository
    /// This returns a single autolink reference by ID that was configured for the given repository.
    ///
    /// Information about autolinks are only available to repository administrators.
    GetReposownerrepoAutolinksautolinkId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/autolinks/{autolink_id}`
    /// * docs https://docs.github.com/v3/repos#delete-autolink
    ///
    /// Delete an autolink reference from a repository
    /// This deletes a single autolink reference by ID that was configured for the given repository.
    ///
    /// Information about autolinks are only available to repository administrators.
    DeleteReposownerrepoAutolinksautolinkId(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/automated-security-fixes`
    /// * docs https://docs.github.com/rest/reference/repos#enable-automated-security-fixes
    ///
    /// Enable automated security fixes
    /// Enables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://help.github.com/en/articles/configuring-automated-security-fixes)".
    PutReposownerrepoAutomatedSecurityFixes(String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/automated-security-fixes`
    /// * docs https://docs.github.com/rest/reference/repos#disable-automated-security-fixes
    ///
    /// Disable automated security fixes
    /// Disables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://help.github.com/en/articles/configuring-automated-security-fixes)".
    DeleteReposownerrepoAutomatedSecurityFixes(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches`
    /// * docs https://docs.github.com/rest/reference/repos#list-branches
    ///
    /// List branches
    ///
    GetReposownerrepoBranches(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-branch
    ///
    /// Get a branch
    ///
    GetReposownerrepoBranchesbranch(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection`
    /// * docs https://docs.github.com/rest/reference/repos#get-branch-protection
    ///
    /// Get branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoBranchesbranchProtection(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection`
    /// * docs https://docs.github.com/rest/reference/repos#update-branch-protection
    ///
    /// Update branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Protecting a branch requires admin or owner permissions to the repository.
    ///
    /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
    ///
    /// **Note**: The list of users, apps, and teams in total is limited to 100 items.
    PutReposownerrepoBranchesbranchProtection(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection`
    /// * docs https://docs.github.com/rest/reference/repos#delete-branch-protection
    ///
    /// Delete branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    DeleteReposownerrepoBranchesbranchProtection(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins`
    /// * docs https://docs.github.com/rest/reference/repos#get-admin-branch-protection
    ///
    /// Get admin branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoBranchesbranchProtectionEnforceAdmins(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins`
    /// * docs https://docs.github.com/rest/reference/repos#set-admin-branch-protection
    ///
    /// Set admin branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
    PostReposownerrepoBranchesbranchProtectionEnforceAdmins(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins`
    /// * docs https://docs.github.com/rest/reference/repos#delete-admin-branch-protection
    ///
    /// Delete admin branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Removing admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
    DeleteReposownerrepoBranchesbranchProtectionEnforceAdmins(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews`
    /// * docs https://docs.github.com/rest/reference/repos#get-pull-request-review-protection
    ///
    /// Get pull request review protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews`
    /// * docs https://docs.github.com/rest/reference/repos#update-pull-request-review-protection
    ///
    /// Update pull request review protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
    ///
    /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
    PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews`
    /// * docs https://docs.github.com/rest/reference/repos#delete-pull-request-review-protection
    ///
    /// Delete pull request review protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    DeleteReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures`
    /// * docs https://docs.github.com/rest/reference/repos#get-commit-signature-protection
    ///
    /// Get commit signature protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://help.github.com/articles/signing-commits-with-gpg) in GitHub Help.
    ///
    /// **Note**: You must enable branch protection to require signed commits.
    GetReposownerrepoBranchesbranchProtectionRequiredSignatures(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures`
    /// * docs https://docs.github.com/rest/reference/repos#create-commit-signature-protection
    ///
    /// Create commit signature protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.
    PostReposownerrepoBranchesbranchProtectionRequiredSignatures(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures`
    /// * docs https://docs.github.com/rest/reference/repos#delete-commit-signature-protection
    ///
    /// Delete commit signature protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to disable required signed commits on a branch. You must enable branch protection to require signed commits.
    DeleteReposownerrepoBranchesbranchProtectionRequiredSignatures(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks`
    /// * docs https://docs.github.com/rest/reference/repos#get-status-checks-protection
    ///
    /// Get status checks protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks`
    /// * docs https://docs.github.com/rest/reference/repos#update-status-check-protection
    ///
    /// Update status check protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.
    PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks`
    /// * docs https://docs.github.com/rest/reference/repos#remove-status-check-protection
    ///
    /// Remove status check protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecks(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts`
    /// * docs https://docs.github.com/rest/reference/repos#get-all-status-check-contexts
    ///
    /// Get all status check contexts
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts`
    /// * docs https://docs.github.com/rest/reference/repos#add-status-check-contexts
    ///
    /// Add status check contexts
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    PostReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts`
    /// * docs https://docs.github.com/rest/reference/repos#set-status-check-contexts
    ///
    /// Set status check contexts
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    PutReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts`
    /// * docs https://docs.github.com/rest/reference/repos#remove-status-check-contexts
    ///
    /// Remove status check contexts
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(
        String,
        String,
        String,
    ),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions`
    /// * docs https://docs.github.com/rest/reference/repos#get-access-restrictions
    ///
    /// Get access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists who has access to this protected branch.
    ///
    /// **Note**: Users, apps, and teams `restrictions` are only available for organization-owned repositories.
    GetReposownerrepoBranchesbranchProtectionRestrictions(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions`
    /// * docs https://docs.github.com/rest/reference/repos#delete-access-restrictions
    ///
    /// Delete access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Disables the ability to restrict who can push to this branch.
    DeleteReposownerrepoBranchesbranchProtectionRestrictions(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps`
    /// * docs https://docs.github.com/rest/reference/repos#list-apps-with-access-to-the-protected-branch
    ///
    /// Get apps with access to the protected branch
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists the GitHub Apps that have push access to this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
    GetReposownerrepoBranchesbranchProtectionRestrictionsApps(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps`
    /// * docs https://docs.github.com/rest/reference/repos#add-app-access-restrictions
    ///
    /// Add app access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Grants the specified apps push access for this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
    ///
    /// | Type    | Description                                                                                                                                                |
    /// | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PostReposownerrepoBranchesbranchProtectionRestrictionsApps(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps`
    /// * docs https://docs.github.com/rest/reference/repos#set-app-access-restrictions
    ///
    /// Set app access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Replaces the list of apps that have push access to this branch. This removes all apps that previously had push access and grants push access to the new list of apps. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
    ///
    /// | Type    | Description                                                                                                                                                |
    /// | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PutReposownerrepoBranchesbranchProtectionRestrictionsApps(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps`
    /// * docs https://docs.github.com/rest/reference/repos#remove-app-access-restrictions
    ///
    /// Remove app access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Removes the ability of an app to push to this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
    ///
    /// | Type    | Description                                                                                                                                                |
    /// | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    DeleteReposownerrepoBranchesbranchProtectionRestrictionsApps(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams`
    /// * docs https://docs.github.com/rest/reference/repos#list-teams-with-access-to-the-protected-branch
    ///
    /// Get teams with access to the protected branch
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists the teams who have push access to this branch. The list includes child teams.
    GetReposownerrepoBranchesbranchProtectionRestrictionsTeams(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams`
    /// * docs https://docs.github.com/rest/reference/repos#add-team-access-restrictions
    ///
    /// Add team access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Grants the specified teams push access for this branch. You can also give push access to child teams.
    ///
    /// | Type    | Description                                                                                                                                |
    /// | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
    /// | `array` | The teams that can have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PostReposownerrepoBranchesbranchProtectionRestrictionsTeams(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams`
    /// * docs https://docs.github.com/rest/reference/repos#set-team-access-restrictions
    ///
    /// Set team access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Replaces the list of teams that have push access to this branch. This removes all teams that previously had push access and grants push access to the new list of teams. Team restrictions include child teams.
    ///
    /// | Type    | Description                                                                                                                                |
    /// | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
    /// | `array` | The teams that can have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PutReposownerrepoBranchesbranchProtectionRestrictionsTeams(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams`
    /// * docs https://docs.github.com/rest/reference/repos#remove-team-access-restrictions
    ///
    /// Remove team access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Removes the ability of a team to push to this branch. You can also remove push access for child teams.
    ///
    /// | Type    | Description                                                                                                                                         |
    /// | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | Teams that should no longer have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    DeleteReposownerrepoBranchesbranchProtectionRestrictionsTeams(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users`
    /// * docs https://docs.github.com/rest/reference/repos#list-users-with-access-to-the-protected-branch
    ///
    /// Get users with access to the protected branch
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists the people who have push access to this branch.
    GetReposownerrepoBranchesbranchProtectionRestrictionsUsers(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users`
    /// * docs https://docs.github.com/rest/reference/repos#add-user-access-restrictions
    ///
    /// Add user access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Grants the specified people push access for this branch.
    ///
    /// | Type    | Description                                                                                                                   |
    /// | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PostReposownerrepoBranchesbranchProtectionRestrictionsUsers(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users`
    /// * docs https://docs.github.com/rest/reference/repos#set-user-access-restrictions
    ///
    /// Set user access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Replaces the list of people that have push access to this branch. This removes all people that previously had push access and grants push access to the new list of people.
    ///
    /// | Type    | Description                                                                                                                   |
    /// | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    PutReposownerrepoBranchesbranchProtectionRestrictionsUsers(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users`
    /// * docs https://docs.github.com/rest/reference/repos#remove-user-access-restrictions
    ///
    /// Remove user access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Removes the ability of a user to push to this branch.
    ///
    /// | Type    | Description                                                                                                                                   |
    /// | ------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `array` | Usernames of the people who should no longer have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
    DeleteReposownerrepoBranchesbranchProtectionRestrictionsUsers(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/rename`
    /// * docs https://docs.github.com/rest/reference/repos#rename-a-branch
    ///
    /// Rename a branch
    /// Renames a branch in a repository.
    ///
    /// **Note:** Although the API responds immediately, the branch rename process might take some extra time to complete in the background. You won't be able to push to the old branch name while the rename process is in progress. For more information, see "[Renaming a branch](https://docs.github.com/github/administering-a-repository/renaming-a-branch)".
    ///
    /// The permissions required to use this endpoint depends on whether you are renaming the default branch.
    ///
    /// To rename a non-default branch:
    ///
    /// * Users must have push access.
    /// * GitHub Apps must have the `contents:write` repository permission.
    ///
    /// To rename the default branch:
    ///
    /// * Users must have admin or owner permissions.
    /// * GitHub Apps must have the `administration:write` repository permission.
    PostReposownerrepoBranchesbranchRename(String, String, String),
    /// * tags checks
    /// * post `/repos/{owner}/{repo}/check-runs`
    /// * docs https://docs.github.com/rest/reference/checks#create-a-check-run
    ///
    /// Create a check run
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Creates a new check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to create check runs.
    ///
    /// In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.
    PostReposownerrepoCheckRuns(String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-runs/{check_run_id}`
    /// * docs https://docs.github.com/rest/reference/checks#get-a-check-run
    ///
    /// Get a check run
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Gets a single check run using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    GetReposownerrepoCheckRunscheckRunId(String, String, String),
    /// * tags checks
    /// * patch `/repos/{owner}/{repo}/check-runs/{check_run_id}`
    /// * docs https://docs.github.com/rest/reference/checks#update-a-check-run
    ///
    /// Update a check run
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Updates a check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to edit check runs.
    PatchReposownerrepoCheckRunscheckRunId(String, String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations`
    /// * docs https://docs.github.com/rest/reference/checks#list-check-run-annotations
    ///
    /// List check run annotations
    /// Lists annotations for a check run using the annotation `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get annotations for a check run. OAuth Apps and authenticated users must have the `repo` scope to get annotations for a check run in a private repository.
    GetReposownerrepoCheckRunscheckRunIdAnnotations(String, String, String),
    /// * tags checks
    /// * post `/repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest`
    /// * docs https://docs.github.com/rest/reference/checks#rerequest-a-check-run
    ///
    /// Rerequest a check run
    /// Triggers GitHub to rerequest an existing check run, without pushing new code to a repository. This endpoint will trigger the [`check_run` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) event with the action `rerequested`. When a check run is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.
    ///
    /// To rerequest a check run, your GitHub App must have the `checks:read` permission on a private repository or pull access to a public repository.
    PostReposownerrepoCheckRunscheckRunIdRerequest(String, String, String),
    /// * tags checks
    /// * post `/repos/{owner}/{repo}/check-suites`
    /// * docs https://docs.github.com/rest/reference/checks#create-a-check-suite
    ///
    /// Create a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/reference/checks#check-runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using "[Update repository preferences for check suites](https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites)". Your GitHub App must have the `checks:write` permission to create check suites.
    PostReposownerrepoCheckSuites(String, String),
    /// * tags checks
    /// * patch `/repos/{owner}/{repo}/check-suites/preferences`
    /// * docs https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites
    ///
    /// Update repository preferences for check suites
    /// Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/reference/checks#create-a-check-suite). You must have admin permissions in the repository to set preferences for check suites.
    PatchReposownerrepoCheckSuitesPreferences(String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-suites/{check_suite_id}`
    /// * docs https://docs.github.com/rest/reference/checks#get-a-check-suite
    ///
    /// Get a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// Gets a single check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
    GetReposownerrepoCheckSuitescheckSuiteId(String, String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs`
    /// * docs https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite
    ///
    /// List check runs in a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Lists check runs for a check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(String, String, String),
    /// * tags checks
    /// * post `/repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest`
    /// * docs https://docs.github.com/rest/reference/checks#rerequest-a-check-suite
    ///
    /// Rerequest a check suite
    /// Triggers GitHub to rerequest an existing check suite, without pushing new code to a repository. This endpoint will trigger the [`check_suite` webhook](https://docs.github.com/webhooks/event-payloads/#check_suite) event with the action `rerequested`. When a check suite is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.
    ///
    /// To rerequest a check suite, your GitHub App must have the `checks:read` permission on a private repository or pull access to a public repository.
    PostReposownerrepoCheckSuitescheckSuiteIdRerequest(String, String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/alerts`
    /// * docs https://docs.github.com/rest/reference/code-scanning#list-code-scanning-alerts-for-a-repository
    ///
    /// List code scanning alerts for a repository
    /// Lists all open code scanning alerts for the default branch (usually `main`
    /// or `master`). You must use an access token with the `security_events` scope to use
    /// this endpoint with private repos, the `public_repo` scope also grants permission to read
    /// security events on public repos only. GitHub Apps must have the `security_events` read
    /// permission to use this endpoint.
    ///
    /// The response includes a `most_recent_instance` object.
    /// This provides details of the most recent instance of this alert
    /// for the default branch or for the specified Git reference
    /// (if you used `ref` in the request).
    GetReposownerrepoCodeScanningAlerts(String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}`
    /// * docs https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-alert
    ///
    /// Get a code scanning alert
    /// Gets a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint with private repos, the `public_repo` scope also grants permission to read security events on public repos only. GitHub Apps must have the `security_events` read permission to use this endpoint.
    ///
    /// **Deprecation notice**:
    /// The instances field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The same information can now be retrieved via a GET request to the URL specified by `instances_url`.
    GetReposownerrepoCodeScanningAlertsalertNumber(String, String, String),
    /// * tags code-scanning
    /// * patch `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}`
    /// * docs https://docs.github.com/rest/reference/code-scanning#update-a-code-scanning-alert
    ///
    /// Update a code scanning alert
    /// Updates the status of a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint with private repositories. You can also use tokens with the `public_repo` scope for public repositories only. GitHub Apps must have the `security_events` write permission to use this endpoint.
    PatchReposownerrepoCodeScanningAlertsalertNumber(String, String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances`
    /// * docs https://docs.github.com/rest/reference/code-scanning#list-instances-of-a-code-scanning-alert
    ///
    /// List instances of a code scanning alert
    /// Lists all instances of the specified code scanning alert.
    /// You must use an access token with the `security_events` scope to use this endpoint with private repos,
    /// the `public_repo` scope also grants permission to read security events on public repos only.
    /// GitHub Apps must have the `security_events` read permission to use this endpoint.
    GetReposownerrepoCodeScanningAlertsalertNumberInstances(String, String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/analyses`
    /// * docs https://docs.github.com/rest/reference/code-scanning#list-code-scanning-analyses-for-a-repository
    ///
    /// List code scanning analyses for a repository
    /// Lists the details of all code scanning analyses for a repository,
    /// starting with the most recent.
    /// The response is paginated and you can use the `page` and `per_page` parameters
    /// to list the analyses you're interested in.
    /// By default 30 analyses are listed per page.
    ///
    /// The `rules_count` field in the response give the number of rules
    /// that were run in the analysis.
    /// For very old analyses this data is not available,
    /// and `0` is returned in this field.
    ///
    /// You must use an access token with the `security_events` scope to use this endpoint with private repos,
    /// the `public_repo` scope also grants permission to read security events on public repos only.
    /// GitHub Apps must have the `security_events` read permission to use this endpoint.
    ///
    /// **Deprecation notice**:
    /// The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
    GetReposownerrepoCodeScanningAnalyses(String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}`
    /// * docs https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository
    ///
    /// Get a code scanning analysis for a repository
    /// Gets a specified code scanning analysis for a repository.
    /// You must use an access token with the `security_events` scope to use this endpoint with private repos,
    /// the `public_repo` scope also grants permission to read security events on public repos only.
    /// GitHub Apps must have the `security_events` read permission to use this endpoint.
    ///
    /// The default JSON response contains fields that describe the analysis.
    /// This includes the Git reference and commit SHA to which the analysis relates,
    /// the datetime of the analysis, the name of the code scanning tool,
    /// and the number of alerts.
    ///
    /// The `rules_count` field in the default response give the number of rules
    /// that were run in the analysis.
    /// For very old analyses this data is not available,
    /// and `0` is returned in this field.
    ///
    /// If you use the Accept header `application/sarif+json`,
    /// the response contains the analysis data that was uploaded.
    /// This is formatted as
    /// [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html).
    GetReposownerrepoCodeScanningAnalysesanalysisId(String, String, String),
    /// * tags code-scanning
    /// * delete `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}`
    /// * docs https://docs.github.com/rest/reference/code-scanning#delete-a-code-scanning-analysis-from-a-repository
    ///
    /// Delete a code scanning analysis from a repository
    /// Deletes a specified code scanning analysis from a repository. For
    /// private repositories, you must use an access token with the `repo` scope. For public repositories,
    /// you must use an access token with `public_repo` scope.
    /// GitHub Apps must have the `security_events` write permission to use this endpoint.
    ///
    /// You can delete one analysis at a time.
    /// To delete a series of analyses, start with the most recent analysis and work backwards.
    /// Conceptually, the process is similar to the undo function in a text editor.
    ///
    /// When you list the analyses for a repository,
    /// one or more will be identified as deletable in the response:
    ///
    /// ```
    /// "deletable": true
    /// ```
    ///
    /// An analysis is deletable when it's the most recent in a set of analyses.
    /// Typically, a repository will have multiple sets of analyses
    /// for each enabled code scanning tool,
    /// where a set is determined by a unique combination of analysis values:
    ///
    /// * `ref`
    /// * `tool`
    /// * `analysis_key`
    /// * `environment`
    ///
    /// If you attempt to delete an analysis that is not the most recent in a set,
    /// you'll get a 400 response with the message:
    ///
    /// ```
    /// Analysis specified is not deletable.
    /// ```
    ///
    /// The response from a successful `DELETE` operation provides you with
    /// two alternative URLs for deleting the next analysis in the set:
    /// `next_analysis_url` and `confirm_delete_url`.
    /// Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
    /// in a set. This is a useful option if you want to preserve at least one analysis
    /// for the specified tool in your repository.
    /// Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
    /// When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url`
    /// in the 200 response is `null`.
    ///
    /// As an example of the deletion process,
    /// let's imagine that you added a workflow that configured a particular code scanning tool
    /// to analyze the code in a repository. This tool has added 15 analyses:
    /// 10 on the default branch, and another 5 on a topic branch.
    /// You therefore have two separate sets of analyses for this tool.
    /// You've now decided that you want to remove all of the analyses for the tool.
    /// To do this you must make 15 separate deletion requests.
    /// To start, you must find an analysis that's identified as deletable.
    /// Each set of analyses always has one that's identified as deletable.
    /// Having found the deletable analysis for one of the two sets,
    /// delete this analysis and then continue deleting the next analysis in the set until they're all deleted.
    /// Then repeat the process for the second set.
    /// The procedure therefore consists of a nested loop:
    ///
    /// **Outer loop**:
    /// * List the analyses for the repository, filtered by tool.
    /// * Parse this list to find a deletable analysis. If found:
    ///
    ///   **Inner loop**:
    ///   * Delete the identified analysis.
    ///   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.
    ///
    /// The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.
    DeleteReposownerrepoCodeScanningAnalysesanalysisId(String, String, String),
    /// * tags code-scanning
    /// * post `/repos/{owner}/{repo}/code-scanning/sarifs`
    /// * docs https://docs.github.com/rest/reference/code-scanning#upload-a-sarif-file
    ///
    /// Upload an analysis as SARIF data
    /// Uploads SARIF data containing the results of a code scanning analysis to make the results available in a repository. You must use an access token with the `security_events` scope to use this endpoint for private repositories. You can also use tokens with the `public_repo` scope for public repositories only. GitHub Apps must have the `security_events` write permission to use this endpoint.
    ///
    /// There are two places where you can upload code scanning results.
    ///  - If you upload to a pull request, for example `--ref refs/pull/42/merge` or `--ref refs/pull/42/head`, then the results appear as alerts in a pull request check. For more information, see "[Triaging code scanning alerts in pull requests](/code-security/secure-coding/triaging-code-scanning-alerts-in-pull-requests)."
    ///  - If you upload to a branch, for example `--ref refs/heads/my-branch`, then the results appear in the **Security** tab for your repository. For more information, see "[Managing code scanning alerts for your repository](/code-security/secure-coding/managing-code-scanning-alerts-for-your-repository#viewing-the-alerts-for-a-repository)."
    ///
    /// You must compress the SARIF-formatted analysis data that you want to upload, using `gzip`, and then encode it as a Base64 format string. For example:
    ///
    /// ```
    /// gzip -c analysis-data.sarif | base64 -w0
    /// ```
    ///
    /// SARIF upload supports a maximum of 5000 results per analysis run. Any results over this limit are ignored and any SARIF uploads with more than 25,000 results are rejected. Typically, but not necessarily, a SARIF file contains a single run of a single tool. If a code scanning tool generates too many results, you should update the analysis configuration to run only the most important rules or queries.
    ///
    /// The `202 Accepted`, response includes an `id` value.
    /// You can use this ID to check the status of the upload by using this for the `/sarifs/{sarif_id}` endpoint.
    /// For more information, see "[Get information about a SARIF upload](/rest/reference/code-scanning#get-information-about-a-sarif-upload)."
    PostReposownerrepoCodeScanningSarifs(String, String),
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}`
    /// * docs https://docs.github.com/rest/reference/code-scanning#list-recent-code-scanning-analyses-for-a-repository
    ///
    /// Get information about a SARIF upload
    /// Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository)." You must use an access token with the `security_events` scope to use this endpoint with private repos, the `public_repo` scope also grants permission to read security events on public repos only. GitHub Apps must have the `security_events` read permission to use this endpoint.
    GetReposownerrepoCodeScanningSarifssarifId(String, String, String),
    /// * tags codespaces
    /// * get `/repos/{owner}/{repo}/codespaces`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-codespaces-in-a-repository-for-the-authenticated-user
    ///
    /// List codespaces in a repository for the authenticated user
    /// Lists the codespaces associated to a specified repository and the authenticated user.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    GetReposownerrepoCodespaces(String, String),
    /// * tags codespaces
    /// * post `/repos/{owner}/{repo}/codespaces`
    /// * docs https://docs.github.com/rest/reference/codespaces#create-a-codespace-in-a-repository
    ///
    /// Create a codespace in a repository
    /// Creates a codespace owned by the authenticated user in the specified repository.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PostReposownerrepoCodespaces(String, String),
    /// * tags codespaces
    /// * get `/repos/{owner}/{repo}/codespaces/machines`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-available-machine-types-for-a-repository
    ///
    /// List available machine types for a repository
    /// List the machine types available for a given repository based on its configuration.
    ///
    /// Location is required.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    GetReposownerrepoCodespacesMachines(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/collaborators`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-collaborators
    ///
    /// List repository collaborators
    /// For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
    ///
    /// Team members will include the members of child teams.
    ///
    /// You must have push access to the repository in order to list collaborators.
    GetReposownerrepoCollaborators(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/repos#check-if-a-user-is-a-repository-collaborator
    ///
    /// Check if a user is a repository collaborator
    /// For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
    ///
    /// Team members will include the members of child teams.
    GetReposownerrepoCollaboratorsusername(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/repos#add-a-repository-collaborator
    ///
    /// Add a repository collaborator
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    ///
    /// For more information on permission levels, see "[Repository permission levels for an organization](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)". There are restrictions on which permissions can be granted to organization members when an organization base role is in place. In this case, the permission being given must be equal to or higher than the org base permission. Otherwise, the request will fail with:
    ///
    /// ```
    /// Cannot assign {member} permission of {role name}
    /// ```
    ///
    /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    ///
    /// The invitee will receive a notification that they have been invited to the repository, which they must accept or decline. They may do this via the notifications page, the email they receive, or by using the [repository invitations API endpoints](https://docs.github.com/rest/reference/repos#invitations).
    ///
    /// **Rate limits**
    ///
    /// You are limited to sending 50 invitations to a repository per 24 hour period. Note there is no limit if you are inviting organization members to an organization repository.
    PutReposownerrepoCollaboratorsusername(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/collaborators/{username}`
    /// * docs https://docs.github.com/rest/reference/repos#remove-a-repository-collaborator
    ///
    /// Remove a repository collaborator
    ///
    DeleteReposownerrepoCollaboratorsusername(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/collaborators/{username}/permission`
    /// * docs https://docs.github.com/rest/reference/repos#get-repository-permissions-for-a-user
    ///
    /// Get repository permissions for a user
    /// Checks the repository permission of a collaborator. The possible repository permissions are `admin`, `write`, `read`, and `none`.
    GetReposownerrepoCollaboratorsusernamePermission(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/comments`
    /// * docs https://docs.github.com/rest/reference/repos#list-commit-comments-for-a-repository
    ///
    /// List commit comments for a repository
    /// Commit Comments use [these custom media types](https://docs.github.com/rest/reference/repos#custom-media-types). You can read more about the use of media types in the API [here](https://docs.github.com/rest/overview/media-types/).
    ///
    /// Comments are ordered by ascending ID.
    GetReposownerrepoComments(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-commit-comment
    ///
    /// Get a commit comment
    ///
    GetReposownerrepoCommentscommentId(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-commit-comment
    ///
    /// Update a commit comment
    ///
    PatchReposownerrepoCommentscommentId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-commit-comment
    ///
    /// Delete a commit comment
    ///
    DeleteReposownerrepoCommentscommentId(String, String, String),
    /// * tags reactions
    /// * get `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-a-commit-comment
    ///
    /// List reactions for a commit comment
    /// List the reactions to a [commit comment](https://docs.github.com/rest/reference/repos#comments).
    GetReposownerrepoCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment
    ///
    /// Create reaction for a commit comment
    /// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
    PostReposownerrepoCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * delete `/repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-a-commit-comment-reaction
    ///
    /// Delete a commit comment reaction
    /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/comments/:comment_id/reactions/:reaction_id`.
    ///
    /// Delete a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments).
    DeleteReposownerrepoCommentscommentIdReactionsreactionId(String, String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits`
    /// * docs https://docs.github.com/rest/reference/repos#list-commits
    ///
    /// List commits
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    GetReposownerrepoCommits(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head`
    /// * docs https://docs.github.com/rest/reference/repos#list-branches-for-head-commit
    ///
    /// List branches for HEAD commit
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
    GetReposownerrepoCommitscommitShaBranchesWhereHead(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
    /// * docs https://docs.github.com/rest/reference/repos#list-commit-comments
    ///
    /// List commit comments
    /// Use the `:commit_sha` to specify the commit that will have its comments listed.
    GetReposownerrepoCommitscommitShaComments(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-commit-comment
    ///
    /// Create a commit comment
    /// Create a comment for a commit using its `:commit_sha`.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoCommitscommitShaComments(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{commit_sha}/pulls`
    /// * docs https://docs.github.com/rest/reference/repos#list-pull-requests-associated-with-a-commit
    ///
    /// List pull requests associated with a commit
    /// Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, additionally returns open pull requests associated with the commit. The results may include open and closed pull requests. Additional preview headers may be required to see certain details for associated pull requests, such as whether a pull request is in a draft state. For more information about previews that might affect this endpoint, see the [List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests) endpoint.
    GetReposownerrepoCommitscommitShaPulls(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{ref}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-commit
    ///
    /// Get a commit
    /// Returns the contents of a single commit reference. You must have `read` access for the repository to use this endpoint.
    ///
    /// **Note:** If there are more than 300 files in the commit diff, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.
    ///
    /// You can pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to  fetch `diff` and `patch` formats. Diffs with binary data will have no `patch` property.
    ///
    /// To return only the SHA-1 hash of the commit reference, you can provide the `sha` custom [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) in the `Accept` header. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    GetReposownerrepoCommitsref(String, String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/commits/{ref}/check-runs`
    /// * docs https://docs.github.com/rest/reference/checks#list-check-runs-for-a-git-reference
    ///
    /// List check runs for a Git reference
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    GetReposownerrepoCommitsrefCheckRuns(String, String, String),
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/commits/{ref}/check-suites`
    /// * docs https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference
    ///
    /// List check suites for a Git reference
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to list check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
    GetReposownerrepoCommitsrefCheckSuites(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{ref}/status`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-combined-status-for-a-specific-reference
    ///
    /// Get the combined status for a specific reference
    /// Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.
    ///
    ///
    /// Additionally, a combined `state` is returned. The `state` is one of:
    ///
    /// *   **failure** if any of the contexts report as `error` or `failure`
    /// *   **pending** if there are no statuses or a context is `pending`
    /// *   **success** if the latest status for all contexts is `success`
    GetReposownerrepoCommitsrefStatus(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{ref}/statuses`
    /// * docs https://docs.github.com/rest/reference/repos#list-commit-statuses-for-a-reference
    ///
    /// List commit statuses for a reference
    /// Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.
    ///
    /// This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.
    GetReposownerrepoCommitsrefStatuses(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/community/profile`
    /// * docs https://docs.github.com/rest/reference/repos#get-community-profile-metrics
    ///
    /// Get community profile metrics
    /// This endpoint will return all community profile metrics, including an
    /// overall health score, repository description, the presence of documentation, detected
    /// code of conduct, detected license, and the presence of ISSUE\_TEMPLATE, PULL\_REQUEST\_TEMPLATE,
    /// README, and CONTRIBUTING files.
    ///
    /// The `health_percentage` score is defined as a percentage of how many of
    /// these four documents are present: README, CONTRIBUTING, LICENSE, and
    /// CODE_OF_CONDUCT. For example, if all four documents are present, then
    /// the `health_percentage` is `100`. If only one is present, then the
    /// `health_percentage` is `25`.
    ///
    /// `content_reports_enabled` is only returned for organization-owned repositories.
    GetReposownerrepoCommunityProfile(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/compare/{basehead}`
    /// * docs https://docs.github.com/rest/reference/repos#compare-two-commits
    ///
    /// Compare two commits
    /// The `basehead` param is comprised of two parts: `base` and `head`. Both must be branch names in `repo`. To compare branches across other repositories in the same network as `repo`, use the format `<USERNAME>:branch`.
    ///
    /// The response from the API is equivalent to running the `git log base..head` command; however, commits are returned in chronological order. Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
    ///
    /// The response also includes details on the files that were changed between the two commits. This includes the status of the change (for example, if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.
    ///
    /// **Working with large comparisons**
    ///
    /// To process a response with a large number of commits, you can use (`per_page` or `page`) to paginate the results. When using paging, the list of changed files is only returned with page 1, but includes all changed files for the entire comparison. For more information on working with pagination, see "[Traversing with pagination](/rest/guides/traversing-with-pagination)."
    ///
    /// When calling this API without any paging parameters (`per_page` or `page`), the returned list is limited to 250 commits and the last commit in the list is the most recent of the entire comparison. When a paging parameter is specified, the first commit in the returned list of each page is the earliest.
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    GetReposownerrepoComparebasehead(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/contents/{path}`
    /// * docs https://docs.github.com/rest/reference/repos#get-repository-content
    ///
    /// Get repository content
    /// Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
    /// `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
    ///
    /// Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
    /// retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
    /// type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
    /// object format.
    ///
    /// **Note**:
    /// *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
    /// *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
    /// API](https://docs.github.com/rest/reference/git#get-a-tree).
    /// *   This API supports files up to 1 megabyte in size.
    ///
    /// #### If the content is a directory
    /// The response will be an array of objects, one object for each item in the directory.
    /// When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
    /// _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
    /// In the next major version of the API, the type will be returned as "submodule".
    ///
    /// #### If the content is a symlink
    /// If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
    /// API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
    /// describing the symlink itself.
    ///
    /// #### If the content is a submodule
    /// The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
    /// commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
    /// the submodule at that specific commit.
    ///
    /// If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
    /// github.com URLs (`html_url` and `_links["html"]`) will have null values.
    GetReposownerrepoContentspath(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/contents/{path}`
    /// * docs https://docs.github.com/rest/reference/repos#create-or-update-file-contents
    ///
    /// Create or update file contents
    /// Creates a new file or replaces an existing file in a repository.
    PutReposownerrepoContentspath(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/contents/{path}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-file
    ///
    /// Delete a file
    /// Deletes a file in a repository.
    ///
    /// You can provide an additional `committer` parameter, which is an object containing information about the committer. Or, you can provide an `author` parameter, which is an object containing information about the author.
    ///
    /// The `author` section is optional and is filled in with the `committer` information if omitted. If the `committer` information is omitted, the authenticated user's information is used.
    ///
    /// You must provide values for both `name` and `email`, whether you choose to use `author` or `committer`. Otherwise, you'll receive a `422` status code.
    DeleteReposownerrepoContentspath(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/contributors`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-contributors
    ///
    /// List repository contributors
    /// Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API v3 caches contributor data to improve performance.
    ///
    /// GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
    GetReposownerrepoContributors(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments`
    /// * docs https://docs.github.com/rest/reference/repos#list-deployments
    ///
    /// List deployments
    /// Simple filtering of deployments is available via query parameters:
    GetReposownerrepoDeployments(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/deployments`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-deployment
    ///
    /// Create a deployment
    /// Deployments offer a few configurable parameters with certain defaults.
    ///
    /// The `ref` parameter can be any named branch, tag, or SHA. At GitHub we often deploy branches and verify them
    /// before we merge a pull request.
    ///
    /// The `environment` parameter allows deployments to be issued to different runtime environments. Teams often have
    /// multiple environments for verifying their applications, such as `production`, `staging`, and `qa`. This parameter
    /// makes it easier to track which environments have requested deployments. The default environment is `production`.
    ///
    /// The `auto_merge` parameter is used to ensure that the requested ref is not behind the repository's default branch. If
    /// the ref _is_ behind the default branch for the repository, we will attempt to merge it for you. If the merge succeeds,
    /// the API will return a successful merge commit. If merge conflicts prevent the merge from succeeding, the API will
    /// return a failure response.
    ///
    /// By default, [commit statuses](https://docs.github.com/rest/reference/repos#statuses) for every submitted context must be in a `success`
    /// state. The `required_contexts` parameter allows you to specify a subset of contexts that must be `success`, or to
    /// specify contexts that have not yet been submitted. You are not required to use commit statuses to deploy. If you do
    /// not require any contexts or create any commit statuses, the deployment will always succeed.
    ///
    /// The `payload` parameter is available for any extra information that a deployment system might need. It is a JSON text
    /// field that will be passed on when a deployment event is dispatched.
    ///
    /// The `task` parameter is used by the deployment system to allow different execution paths. In the web world this might
    /// be `deploy:migrations` to run schema changes on the system. In the compiled world this could be a flag to compile an
    /// application with debugging enabled.
    ///
    /// Users with `repo` or `repo_deployment` scopes can create a deployment for a given ref.
    ///
    /// #### Merged branch response
    /// You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating
    /// a deployment. This auto-merge happens when:
    /// *   Auto-merge option is enabled in the repository
    /// *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example
    /// *   There are no merge conflicts
    ///
    /// If there are no new commits in the base branch, a new request to create a deployment should give a successful
    /// response.
    ///
    /// #### Merge conflict response
    /// This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't
    /// be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.
    ///
    /// #### Failed commit status checks
    /// This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success`
    /// status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.
    PostReposownerrepoDeployments(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments/{deployment_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-deployment
    ///
    /// Get a deployment
    ///
    GetReposownerrepoDeploymentsdeploymentId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/deployments/{deployment_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-deployment
    ///
    /// Delete a deployment
    /// To ensure there can always be an active deployment, you can only delete an _inactive_ deployment. Anyone with `repo` or `repo_deployment` scopes can delete an inactive deployment.
    ///
    /// To set a deployment as inactive, you must:
    ///
    /// *   Create a new deployment that is active so that the system has a record of the current state, then delete the previously active deployment.
    /// *   Mark the active deployment as inactive by adding any non-successful deployment status.
    ///
    /// For more information, see "[Create a deployment](https://docs.github.com/rest/reference/repos/#create-a-deployment)" and "[Create a deployment status](https://docs.github.com/rest/reference/repos#create-a-deployment-status)."
    DeleteReposownerrepoDeploymentsdeploymentId(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses`
    /// * docs https://docs.github.com/rest/reference/repos#list-deployment-statuses
    ///
    /// List deployment statuses
    /// Users with pull access can view deployment statuses for a deployment:
    GetReposownerrepoDeploymentsdeploymentIdStatuses(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-deployment-status
    ///
    /// Create a deployment status
    /// Users with `push` access can create deployment statuses for a given deployment.
    ///
    /// GitHub Apps require `read & write` access to "Deployments" and `read-only` access to "Repo contents" (for private repos). OAuth Apps require the `repo_deployment` scope.
    PostReposownerrepoDeploymentsdeploymentIdStatuses(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-deployment-status
    ///
    /// Get a deployment status
    /// Users with pull access can view a deployment status for a deployment:
    GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(String, String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/dispatches`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-repository-dispatch-event
    ///
    /// Create a repository dispatch event
    /// You can use this endpoint to trigger a webhook event called `repository_dispatch` when you want activity that happens outside of GitHub to trigger a GitHub Actions workflow or GitHub App webhook. You must configure your GitHub Actions workflow or GitHub App to run when the `repository_dispatch` event occurs. For an example `repository_dispatch` webhook payload, see "[RepositoryDispatchEvent](https://docs.github.com/webhooks/event-payloads/#repository_dispatch)."
    ///
    /// The `client_payload` parameter is available for any extra information that your workflow might need. This parameter is a JSON payload that will be passed on when the webhook event is dispatched. For example, the `client_payload` can include a message that a user would like to send using a GitHub Actions workflow. Or the `client_payload` can be used as a test to debug your workflow.
    ///
    /// This endpoint requires write access to the repository by providing either:
    ///
    ///   - Personal access tokens with `repo` scope. For more information, see "[Creating a personal access token for the command line](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line)" in the GitHub Help documentation.
    ///   - GitHub Apps with both `metadata:read` and `contents:read&write` permissions.
    ///
    /// This input example shows how you can use the `client_payload` as a test to debug your workflow.
    PostReposownerrepoDispatches(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/environments`
    /// * docs https://docs.github.com/rest/reference/repos#get-all-environments
    ///
    /// Get all environments
    /// Get all environments for a repository.
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoEnvironments(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/environments/{environment_name}`
    /// * docs https://docs.github.com/rest/reference/repos#get-an-environment
    ///
    /// Get an environment
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    GetReposownerrepoEnvironmentsenvironmentName(String, String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/environments/{environment_name}`
    /// * docs https://docs.github.com/rest/reference/repos#create-or-update-an-environment
    ///
    /// Create or update an environment
    /// Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see "[Environments](/actions/reference/environments#environment-protection-rules)."
    ///
    /// **Note:** Although you can use this operation to specify that only branches that match specified name patterns can deploy to this environment, you must use the UI to set the name patterns. For more information, see "[Environments](/actions/reference/environments#deployment-branches)."
    ///
    /// **Note:** To create or update secrets for an environment, see "[Secrets](/rest/reference/actions#secrets)."
    ///
    /// You must authenticate using an access token with the repo scope to use this endpoint.
    PutReposownerrepoEnvironmentsenvironmentName(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/environments/{environment_name}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-an-environment
    ///
    /// Delete an environment
    /// You must authenticate using an access token with the repo scope to use this endpoint.
    DeleteReposownerrepoEnvironmentsenvironmentName(String, String, String),
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/events`
    /// * docs https://docs.github.com/rest/reference/activity#list-repository-events
    ///
    /// List repository events
    ///
    GetReposownerrepoEvents(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/forks`
    /// * docs https://docs.github.com/rest/reference/repos#list-forks
    ///
    /// List forks
    ///
    GetReposownerrepoForks(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/forks`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-fork
    ///
    /// Create a fork
    /// Create a fork for the authenticated user.
    ///
    /// **Note**: Forking a Repository happens asynchronously. You may have to wait a short period of time before you can access the git objects. If this takes longer than 5 minutes, be sure to contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
    PostReposownerrepoForks(String, String),
    /// * tags git
    /// * post `/repos/{owner}/{repo}/git/blobs`
    /// * docs https://docs.github.com/rest/reference/git#create-a-blob
    ///
    /// Create a blob
    ///
    PostReposownerrepoGitBlobs(String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/blobs/{file_sha}`
    /// * docs https://docs.github.com/rest/reference/git#get-a-blob
    ///
    /// Get a blob
    /// The `content` in the response will always be Base64 encoded.
    ///
    /// _Note_: This API supports blobs up to 100 megabytes in size.
    GetReposownerrepoGitBlobsfileSha(String, String, String),
    /// * tags git
    /// * post `/repos/{owner}/{repo}/git/commits`
    /// * docs https://docs.github.com/rest/reference/git#create-a-commit
    ///
    /// Create a commit
    /// Creates a new Git [commit object](https://git-scm.com/book/en/v1/Git-Internals-Git-Objects#Commit-Objects).
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    PostReposownerrepoGitCommits(String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/commits/{commit_sha}`
    /// * docs https://docs.github.com/rest/reference/git#get-a-commit
    ///
    /// Get a commit
    /// Gets a Git [commit object](https://git-scm.com/book/en/v1/Git-Internals-Git-Objects#Commit-Objects).
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    GetReposownerrepoGitCommitscommitSha(String, String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/matching-refs/{ref}`
    /// * docs https://docs.github.com/rest/reference/git#list-matching-references
    ///
    /// List matching references
    /// Returns an array of references from your Git database that match the supplied name. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't exist in the repository, but existing refs start with `:ref`, they will be returned as an array.
    ///
    /// When you use this endpoint without providing a `:ref`, it will return an array of all the references from your Git database, including notes and stashes if they exist on the server. Anything in the namespace is returned, not just `heads` and `tags`.
    ///
    /// **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
    ///
    /// If you request matching references for a branch named `feature` but the branch `feature` doesn't exist, the response can still include other matching head refs that start with the word `feature`, such as `featureA` and `featureB`.
    GetReposownerrepoGitMatchingRefsref(String, String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/ref/{ref}`
    /// * docs https://docs.github.com/rest/reference/git#get-a-reference
    ///
    /// Get a reference
    /// Returns a single reference from your Git database. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't match an existing ref, a `404` is returned.
    ///
    /// **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
    GetReposownerrepoGitRefref(String, String, String),
    /// * tags git
    /// * post `/repos/{owner}/{repo}/git/refs`
    /// * docs https://docs.github.com/rest/reference/git#create-a-reference
    ///
    /// Create a reference
    /// Creates a reference for your repository. You are unable to create new references for empty repositories, even if the commit SHA-1 hash used exists. Empty repositories are repositories without branches.
    PostReposownerrepoGitRefs(String, String),
    /// * tags git
    /// * patch `/repos/{owner}/{repo}/git/refs/{ref}`
    /// * docs https://docs.github.com/rest/reference/git#update-a-reference
    ///
    /// Update a reference
    ///
    PatchReposownerrepoGitRefsref(String, String, String),
    /// * tags git
    /// * delete `/repos/{owner}/{repo}/git/refs/{ref}`
    /// * docs https://docs.github.com/rest/reference/git#delete-a-reference
    ///
    /// Delete a reference
    ///
    DeleteReposownerrepoGitRefsref(String, String, String),
    /// * tags git
    /// * post `/repos/{owner}/{repo}/git/tags`
    /// * docs https://docs.github.com/rest/reference/git#create-a-tag-object
    ///
    /// Create a tag object
    /// Note that creating a tag object does not create the reference that makes a tag in Git. If you want to create an annotated tag in Git, you have to do this call to create the tag object, and then [create](https://docs.github.com/rest/reference/git#create-a-reference) the `refs/tags/[tag]` reference. If you want to create a lightweight tag, you only have to [create](https://docs.github.com/rest/reference/git#create-a-reference) the tag reference - this call would be unnecessary.
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    PostReposownerrepoGitTags(String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/tags/{tag_sha}`
    /// * docs https://docs.github.com/rest/reference/git#get-a-tag
    ///
    /// Get a tag
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    GetReposownerrepoGitTagstagSha(String, String, String),
    /// * tags git
    /// * post `/repos/{owner}/{repo}/git/trees`
    /// * docs https://docs.github.com/rest/reference/git#create-a-tree
    ///
    /// Create a tree
    /// The tree creation API accepts nested entries. If you specify both a tree and a nested path modifying that tree, this endpoint will overwrite the contents of the tree with the new path contents, and create a new tree structure.
    ///
    /// If you use this endpoint to add, delete, or modify the file contents in a tree, you will need to commit the tree and then update a branch to point to the commit. For more information see "[Create a commit](https://docs.github.com/rest/reference/git#create-a-commit)" and "[Update a reference](https://docs.github.com/rest/reference/git#update-a-reference)."
    PostReposownerrepoGitTrees(String, String),
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/trees/{tree_sha}`
    /// * docs https://docs.github.com/rest/reference/git#get-a-tree
    ///
    /// Get a tree
    /// Returns a single tree using the SHA1 value for that tree.
    ///
    /// If `truncated` is `true` in the response then the number of items in the `tree` array exceeded our maximum limit. If you need to fetch more items, use the non-recursive method of fetching trees, and fetch one sub-tree at a time.
    GetReposownerrepoGitTreestreeSha(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-webhooks
    ///
    /// List repository webhooks
    ///
    GetReposownerrepoHooks(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/hooks`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-repository-webhook
    ///
    /// Create a repository webhook
    /// Repositories can have multiple webhooks installed. Each webhook should have a unique `config`. Multiple webhooks can
    /// share the same `config` as long as those webhooks do not have any `events` that overlap.
    PostReposownerrepoHooks(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-repository-webhook
    ///
    /// Get a repository webhook
    /// Returns a webhook configured in a repository. To get only the webhook `config` properties, see "[Get a webhook configuration for a repository](/rest/reference/repos#get-a-webhook-configuration-for-a-repository)."
    GetReposownerrepoHookshookId(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-repository-webhook
    ///
    /// Update a repository webhook
    /// Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for a repository](/rest/reference/repos#update-a-webhook-configuration-for-a-repository)."
    PatchReposownerrepoHookshookId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/hooks/{hook_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-repository-webhook
    ///
    /// Delete a repository webhook
    ///
    DeleteReposownerrepoHookshookId(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}/config`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-webhook-configuration-for-a-repository
    ///
    /// Get a webhook configuration for a repository
    /// Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use "[Get a repository webhook](/rest/reference/orgs#get-a-repository-webhook)."
    ///
    /// Access tokens must have the `read:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:read` permission.
    GetReposownerrepoHookshookIdConfig(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/hooks/{hook_id}/config`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-webhook-configuration-for-a-repository
    ///
    /// Update a webhook configuration for a repository
    /// Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use "[Update a repository webhook](/rest/reference/orgs#update-a-repository-webhook)."
    ///
    /// Access tokens must have the `write:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:write` permission.
    PatchReposownerrepoHookshookIdConfig(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries`
    /// * docs https://docs.github.com/rest/reference/repos#list-deliveries-for-a-repository-webhook
    ///
    /// List deliveries for a repository webhook
    /// Returns a list of webhook deliveries for a webhook configured in a repository.
    GetReposownerrepoHookshookIdDeliveries(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-delivery-for-a-repository-webhook
    ///
    /// Get a delivery for a repository webhook
    /// Returns a delivery for a webhook configured in a repository.
    GetReposownerrepoHookshookIdDeliveriesdeliveryId(String, String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}/attempts`
    /// * docs https://docs.github.com/rest/reference/repos#redeliver-a-delivery-for-a-repository-webhook
    ///
    /// Redeliver a delivery for a repository webhook
    /// Redeliver a webhook delivery for a webhook configured in a repository.
    PostReposownerrepoHookshookIdDeliveriesdeliveryIdAttempts(String, String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/hooks/{hook_id}/pings`
    /// * docs https://docs.github.com/rest/reference/repos#ping-a-repository-webhook
    ///
    /// Ping a repository webhook
    /// This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.
    PostReposownerrepoHookshookIdPings(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/hooks/{hook_id}/tests`
    /// * docs https://docs.github.com/rest/reference/repos#test-the-push-repository-webhook
    ///
    /// Test the push repository webhook
    /// This will trigger the hook with the latest push to the current repository if the hook is subscribed to `push` events. If the hook is not subscribed to `push` events, the server will respond with 204 but no test POST will be generated.
    ///
    /// **Note**: Previously `/repos/:owner/:repo/hooks/:hook_id/test`
    PostReposownerrepoHookshookIdTests(String, String, String),
    /// * tags migrations
    /// * get `/repos/{owner}/{repo}/import`
    /// * docs https://docs.github.com/rest/reference/migrations#get-an-import-status
    ///
    /// Get an import status
    /// View the progress of an import.
    ///
    /// **Import status**
    ///
    /// This section includes details about the possible values of the `status` field of the Import Progress response.
    ///
    /// An import that does not have errors will progress through these steps:
    ///
    /// *   `detecting` - the "detection" step of the import is in progress because the request did not include a `vcs` parameter. The import is identifying the type of source control present at the URL.
    /// *   `importing` - the "raw" step of the import is in progress. This is where commit data is fetched from the original repository. The import progress response will include `commit_count` (the total number of raw commits that will be imported) and `percent` (0 - 100, the current progress through the import).
    /// *   `mapping` - the "rewrite" step of the import is in progress. This is where SVN branches are converted to Git branches, and where author updates are applied. The import progress response does not include progress information.
    /// *   `pushing` - the "push" step of the import is in progress. This is where the importer updates the repository on GitHub. The import progress response will include `push_percent`, which is the percent value reported by `git push` when it is "Writing objects".
    /// *   `complete` - the import is complete, and the repository is ready on GitHub.
    ///
    /// If there are problems, you will see one of these in the `status` field:
    ///
    /// *   `auth_failed` - the import requires authentication in order to connect to the original repository. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    /// *   `error` - the import encountered an error. The import progress response will include the `failed_step` and an error message. Contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api) for more information.
    /// *   `detection_needs_auth` - the importer requires authentication for the originating repository to continue detection. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    /// *   `detection_found_nothing` - the importer didn't recognize any source control at the URL. To resolve, [Cancel the import](https://docs.github.com/rest/reference/migrations#cancel-an-import) and [retry](https://docs.github.com/rest/reference/migrations#start-an-import) with the correct URL.
    /// *   `detection_found_multiple` - the importer found several projects or repositories at the provided URL. When this is the case, the Import Progress response will also include a `project_choices` field with the possible project choices as values. To update project choice, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    ///
    /// **The project_choices field**
    ///
    /// When multiple projects are found at the provided URL, the response hash will include a `project_choices` field, the value of which is an array of hashes each representing a project choice. The exact key/value pairs of the project hashes will differ depending on the version control type.
    ///
    /// **Git LFS related fields**
    ///
    /// This section includes details about Git LFS related fields that may be present in the Import Progress response.
    ///
    /// *   `use_lfs` - describes whether the import has been opted in or out of using Git LFS. The value can be `opt_in`, `opt_out`, or `undecided` if no action has been taken.
    /// *   `has_large_files` - the boolean value describing whether files larger than 100MB were found during the `importing` step.
    /// *   `large_files_size` - the total size in gigabytes of files larger than 100MB found in the originating repository.
    /// *   `large_files_count` - the total number of files larger than 100MB found in the originating repository. To see a list of these files, make a "Get Large Files" request.
    GetReposownerrepoImport(String, String),
    /// * tags migrations
    /// * put `/repos/{owner}/{repo}/import`
    /// * docs https://docs.github.com/rest/reference/migrations#start-an-import
    ///
    /// Start an import
    /// Start a source import to a GitHub repository using GitHub Importer.
    PutReposownerrepoImport(String, String),
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import`
    /// * docs https://docs.github.com/rest/reference/migrations#update-an-import
    ///
    /// Update an import
    /// An import can be updated with credentials or a project choice by passing in the appropriate parameters in this API
    /// request. If no parameters are provided, the import will be restarted.
    PatchReposownerrepoImport(String, String),
    /// * tags migrations
    /// * delete `/repos/{owner}/{repo}/import`
    /// * docs https://docs.github.com/rest/reference/migrations#cancel-an-import
    ///
    /// Cancel an import
    /// Stop an import for a repository.
    DeleteReposownerrepoImport(String, String),
    /// * tags migrations
    /// * get `/repos/{owner}/{repo}/import/authors`
    /// * docs https://docs.github.com/rest/reference/migrations#get-commit-authors
    ///
    /// Get commit authors
    /// Each type of source control system represents authors in a different way. For example, a Git commit author has a display name and an email address, but a Subversion commit author just has a username. The GitHub Importer will make the author information valid, but the author might not be correct. For example, it will change the bare Subversion username `hubot` into something like `hubot <hubot@12341234-abab-fefe-8787-fedcba987654>`.
    ///
    /// This endpoint and the [Map a commit author](https://docs.github.com/rest/reference/migrations#map-a-commit-author) endpoint allow you to provide correct Git author information.
    GetReposownerrepoImportAuthors(String, String),
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import/authors/{author_id}`
    /// * docs https://docs.github.com/rest/reference/migrations#map-a-commit-author
    ///
    /// Map a commit author
    /// Update an author's identity for the import. Your application can continue updating authors any time before you push new commits to the repository.
    PatchReposownerrepoImportAuthorsauthorId(String, String, String),
    /// * tags migrations
    /// * get `/repos/{owner}/{repo}/import/large_files`
    /// * docs https://docs.github.com/rest/reference/migrations#get-large-files
    ///
    /// Get large files
    /// List files larger than 100MB found during the import
    GetReposownerrepoImportLargeFiles(String, String),
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import/lfs`
    /// * docs https://docs.github.com/rest/reference/migrations#update-git-lfs-preference
    ///
    /// Update Git LFS preference
    /// You can import repositories from Subversion, Mercurial, and TFS that include files larger than 100MB. This ability is powered by [Git LFS](https://git-lfs.github.com). You can learn more about our LFS feature and working with large files [on our help site](https://help.github.com/articles/versioning-large-files/).
    PatchReposownerrepoImportLfs(String, String),
    /// * tags apps
    /// * get `/repos/{owner}/{repo}/installation`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-repository-installation-for-the-authenticated-app
    ///
    /// Get a repository installation for the authenticated app
    /// Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetReposownerrepoInstallation(String, String),
    /// * tags interactions
    /// * get `/repos/{owner}/{repo}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-a-repository
    ///
    /// Get interaction restrictions for a repository
    /// Shows which type of GitHub user can interact with this repository and when the restriction expires. If there are no restrictions, you will see an empty response.
    GetReposownerrepoInteractionLimits(String, String),
    /// * tags interactions
    /// * put `/repos/{owner}/{repo}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-a-repository
    ///
    /// Set interaction restrictions for a repository
    /// Temporarily restricts interactions to a certain type of GitHub user within the given repository. You must have owner or admin access to set these restrictions. If an interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
    PutReposownerrepoInteractionLimits(String, String),
    /// * tags interactions
    /// * delete `/repos/{owner}/{repo}/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-for-a-repository
    ///
    /// Remove interaction restrictions for a repository
    /// Removes all interaction restrictions from the given repository. You must have owner or admin access to remove restrictions. If the interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
    DeleteReposownerrepoInteractionLimits(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/invitations`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-invitations
    ///
    /// List repository invitations
    /// When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
    GetReposownerrepoInvitations(String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/invitations/{invitation_id}`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-repository-invitation
    ///
    /// Update a repository invitation
    ///
    PatchReposownerrepoInvitationsinvitationId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/invitations/{invitation_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-repository-invitation
    ///
    /// Delete a repository invitation
    ///
    DeleteReposownerrepoInvitationsinvitationId(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues`
    /// * docs https://docs.github.com/rest/reference/issues#list-repository-issues
    ///
    /// List repository issues
    /// List issues in a repository.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetReposownerrepoIssues(String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/issues`
    /// * docs https://docs.github.com/rest/reference/issues#create-an-issue
    ///
    /// Create an issue
    /// Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://help.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoIssues(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/comments`
    /// * docs https://docs.github.com/rest/reference/issues#list-issue-comments-for-a-repository
    ///
    /// List issue comments for a repository
    /// By default, Issue Comments are ordered by ascending ID.
    GetReposownerrepoIssuesComments(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/issues#get-an-issue-comment
    ///
    /// Get an issue comment
    ///
    GetReposownerrepoIssuesCommentscommentId(String, String, String),
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/issues/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/issues#update-an-issue-comment
    ///
    /// Update an issue comment
    ///
    PatchReposownerrepoIssuesCommentscommentId(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/issues#delete-an-issue-comment
    ///
    /// Delete an issue comment
    ///
    DeleteReposownerrepoIssuesCommentscommentId(String, String, String),
    /// * tags reactions
    /// * get `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue-comment
    ///
    /// List reactions for an issue comment
    /// List the reactions to an [issue comment](https://docs.github.com/rest/reference/issues#comments).
    GetReposownerrepoIssuesCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue-comment
    ///
    /// Create reaction for an issue comment
    /// Create a reaction to an [issue comment](https://docs.github.com/rest/reference/issues#comments). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.
    PostReposownerrepoIssuesCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * delete `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-an-issue-comment-reaction
    ///
    /// Delete an issue comment reaction
    /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/issues/comments/:comment_id/reactions/:reaction_id`.
    ///
    /// Delete a reaction to an [issue comment](https://docs.github.com/rest/reference/issues#comments).
    DeleteReposownerrepoIssuesCommentscommentIdReactionsreactionId(String, String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/events`
    /// * docs https://docs.github.com/rest/reference/issues#list-issue-events-for-a-repository
    ///
    /// List issue events for a repository
    ///
    GetReposownerrepoIssuesEvents(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/events/{event_id}`
    /// * docs https://docs.github.com/rest/reference/issues#get-an-issue-event
    ///
    /// Get an issue event
    ///
    GetReposownerrepoIssuesEventseventId(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}`
    /// * docs https://docs.github.com/rest/reference/issues#get-an-issue
    ///
    /// Get an issue
    /// The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-redirects-redirects) if the issue was
    /// [transferred](https://help.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
    /// the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
    /// returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
    /// access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
    /// to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetReposownerrepoIssuesissueNumber(String, String, String),
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/issues/{issue_number}`
    /// * docs https://docs.github.com/rest/reference/issues/#update-an-issue
    ///
    /// Update an issue
    /// Issue owners and users with push access can edit an issue.
    PatchReposownerrepoIssuesissueNumber(String, String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/issues/{issue_number}/assignees`
    /// * docs https://docs.github.com/rest/reference/issues#add-assignees-to-an-issue
    ///
    /// Add assignees to an issue
    /// Adds up to 10 assignees to an issue. Users already assigned to an issue are not replaced.
    PostReposownerrepoIssuesissueNumberAssignees(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/assignees`
    /// * docs https://docs.github.com/rest/reference/issues#remove-assignees-from-an-issue
    ///
    /// Remove assignees from an issue
    /// Removes one or more assignees from an issue.
    DeleteReposownerrepoIssuesissueNumberAssignees(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}/comments`
    /// * docs https://docs.github.com/rest/reference/issues#list-issue-comments
    ///
    /// List issue comments
    /// Issue Comments are ordered by ascending ID.
    GetReposownerrepoIssuesissueNumberComments(String, String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/issues/{issue_number}/comments`
    /// * docs https://docs.github.com/rest/reference/issues#create-an-issue-comment
    ///
    /// Create an issue comment
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoIssuesissueNumberComments(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}/events`
    /// * docs https://docs.github.com/rest/reference/issues#list-issue-events
    ///
    /// List issue events
    ///
    GetReposownerrepoIssuesissueNumberEvents(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#list-labels-for-an-issue
    ///
    /// List labels for an issue
    ///
    GetReposownerrepoIssuesissueNumberLabels(String, String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/issues/{issue_number}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#add-labels-to-an-issue
    ///
    /// Add labels to an issue
    ///
    PostReposownerrepoIssuesissueNumberLabels(String, String, String),
    /// * tags issues
    /// * put `/repos/{owner}/{repo}/issues/{issue_number}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#set-labels-for-an-issue
    ///
    /// Set labels for an issue
    /// Removes any previous labels and sets the new labels for an issue.
    PutReposownerrepoIssuesissueNumberLabels(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#remove-all-labels-from-an-issue
    ///
    /// Remove all labels from an issue
    ///
    DeleteReposownerrepoIssuesissueNumberLabels(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/issues#remove-a-label-from-an-issue
    ///
    /// Remove a label from an issue
    /// Removes the specified label from the issue, and returns the remaining labels on the issue. This endpoint returns a `404 Not Found` status if the label does not exist.
    DeleteReposownerrepoIssuesissueNumberLabelsname(String, String, String, String),
    /// * tags issues
    /// * put `/repos/{owner}/{repo}/issues/{issue_number}/lock`
    /// * docs https://docs.github.com/rest/reference/issues#lock-an-issue
    ///
    /// Lock an issue
    /// Users with push access can lock an issue or pull request's conversation.
    ///
    /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutReposownerrepoIssuesissueNumberLock(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/lock`
    /// * docs https://docs.github.com/rest/reference/issues#unlock-an-issue
    ///
    /// Unlock an issue
    /// Users with push access can unlock an issue's conversation.
    DeleteReposownerrepoIssuesissueNumberLock(String, String, String),
    /// * tags reactions
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue
    ///
    /// List reactions for an issue
    /// List the reactions to an [issue](https://docs.github.com/rest/reference/issues).
    GetReposownerrepoIssuesissueNumberReactions(String, String, String),
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/issues/{issue_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue
    ///
    /// Create reaction for an issue
    /// Create a reaction to an [issue](https://docs.github.com/rest/reference/issues/). A response with an HTTP `200` status means that you already added the reaction type to this issue.
    PostReposownerrepoIssuesissueNumberReactions(String, String, String),
    /// * tags reactions
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-an-issue-reaction
    ///
    /// Delete an issue reaction
    /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/issues/:issue_number/reactions/:reaction_id`.
    ///
    /// Delete a reaction to an [issue](https://docs.github.com/rest/reference/issues/).
    DeleteReposownerrepoIssuesissueNumberReactionsreactionId(String, String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}/timeline`
    /// * docs https://docs.github.com/rest/reference/issues#list-timeline-events-for-an-issue
    ///
    /// List timeline events for an issue
    ///
    GetReposownerrepoIssuesissueNumberTimeline(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/keys`
    /// * docs https://docs.github.com/rest/reference/repos#list-deploy-keys
    ///
    /// List deploy keys
    ///
    GetReposownerrepoKeys(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/keys`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-deploy-key
    ///
    /// Create a deploy key
    /// You can create a read-only deploy key.
    PostReposownerrepoKeys(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/keys/{key_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-deploy-key
    ///
    /// Get a deploy key
    ///
    GetReposownerrepoKeyskeyId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/keys/{key_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-deploy-key
    ///
    /// Delete a deploy key
    /// Deploy keys are immutable. If you need to update a key, remove the key and create a new one instead.
    DeleteReposownerrepoKeyskeyId(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#list-labels-for-a-repository
    ///
    /// List labels for a repository
    ///
    GetReposownerrepoLabels(String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#create-a-label
    ///
    /// Create a label
    ///
    PostReposownerrepoLabels(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/issues#get-a-label
    ///
    /// Get a label
    ///
    GetReposownerrepoLabelsname(String, String, String),
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/issues#update-a-label
    ///
    /// Update a label
    ///
    PatchReposownerrepoLabelsname(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/labels/{name}`
    /// * docs https://docs.github.com/rest/reference/issues#delete-a-label
    ///
    /// Delete a label
    ///
    DeleteReposownerrepoLabelsname(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/languages`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-languages
    ///
    /// List repository languages
    /// Lists languages for the specified repository. The value shown for each language is the number of bytes of code written in that language.
    GetReposownerrepoLanguages(String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/lfs`
    /// * docs https://docs.github.com/rest/reference/repos#enable-git-lfs-for-a-repository
    ///
    /// Enable Git LFS for a repository
    ///
    PutReposownerrepoLfs(String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/lfs`
    /// * docs https://docs.github.com/rest/reference/repos#disable-git-lfs-for-a-repository
    ///
    /// Disable Git LFS for a repository
    ///
    DeleteReposownerrepoLfs(String, String),
    /// * tags licenses
    /// * get `/repos/{owner}/{repo}/license`
    /// * docs https://docs.github.com/rest/reference/licenses/#get-the-license-for-a-repository
    ///
    /// Get the license for a repository
    /// This method returns the contents of the repository's license file, if one is detected.
    ///
    /// Similar to [Get repository content](https://docs.github.com/rest/reference/repos#get-repository-content), this method also supports [custom media types](https://docs.github.com/rest/overview/media-types) for retrieving the raw license content or rendered license HTML.
    GetReposownerrepoLicense(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/merge-upstream`
    /// * docs https://docs.github.com/rest/reference/repos#sync-a-fork-branch-with-the-upstream-repository
    ///
    /// Sync a fork branch with the upstream repository
    /// Sync a branch of a forked repository to keep it up-to-date with the upstream repository.
    PostReposownerrepoMergeUpstream(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/merges`
    /// * docs https://docs.github.com/rest/reference/repos#merge-a-branch
    ///
    /// Merge a branch
    ///
    PostReposownerrepoMerges(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/milestones`
    /// * docs https://docs.github.com/rest/reference/issues#list-milestones
    ///
    /// List milestones
    ///
    GetReposownerrepoMilestones(String, String),
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/milestones`
    /// * docs https://docs.github.com/rest/reference/issues#create-a-milestone
    ///
    /// Create a milestone
    ///
    PostReposownerrepoMilestones(String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/milestones/{milestone_number}`
    /// * docs https://docs.github.com/rest/reference/issues#get-a-milestone
    ///
    /// Get a milestone
    ///
    GetReposownerrepoMilestonesmilestoneNumber(String, String, String),
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/milestones/{milestone_number}`
    /// * docs https://docs.github.com/rest/reference/issues#update-a-milestone
    ///
    /// Update a milestone
    ///
    PatchReposownerrepoMilestonesmilestoneNumber(String, String, String),
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/milestones/{milestone_number}`
    /// * docs https://docs.github.com/rest/reference/issues#delete-a-milestone
    ///
    /// Delete a milestone
    ///
    DeleteReposownerrepoMilestonesmilestoneNumber(String, String, String),
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/milestones/{milestone_number}/labels`
    /// * docs https://docs.github.com/rest/reference/issues#list-labels-for-issues-in-a-milestone
    ///
    /// List labels for issues in a milestone
    ///
    GetReposownerrepoMilestonesmilestoneNumberLabels(String, String, String),
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/notifications`
    /// * docs https://docs.github.com/rest/reference/activity#list-repository-notifications-for-the-authenticated-user
    ///
    /// List repository notifications for the authenticated user
    /// List all notifications for the current user.
    GetReposownerrepoNotifications(String, String),
    /// * tags activity
    /// * put `/repos/{owner}/{repo}/notifications`
    /// * docs https://docs.github.com/rest/reference/activity#mark-repository-notifications-as-read
    ///
    /// Mark repository notifications as read
    /// Marks all notifications in a repository as "read" removes them from the [default view on GitHub](https://github.com/notifications). If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List repository notifications for the authenticated user](https://docs.github.com/rest/reference/activity#list-repository-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
    PutReposownerrepoNotifications(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-github-pages-site
    ///
    /// Get a GitHub Pages site
    ///
    GetReposownerrepoPages(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/pages`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-github-pages-site
    ///
    /// Create a GitHub Pages site
    /// Configures a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages)."
    PostReposownerrepoPages(String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/pages`
    /// * docs https://docs.github.com/rest/reference/repos#update-information-about-a-github-pages-site
    ///
    /// Update information about a GitHub Pages site
    /// Updates information for a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages).
    PutReposownerrepoPages(String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/pages`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-github-pages-site
    ///
    /// Delete a GitHub Pages site
    ///
    DeleteReposownerrepoPages(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/builds`
    /// * docs https://docs.github.com/rest/reference/repos#list-github-pages-builds
    ///
    /// List GitHub Pages builds
    ///
    GetReposownerrepoPagesBuilds(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/pages/builds`
    /// * docs https://docs.github.com/rest/reference/repos#request-a-github-pages-build
    ///
    /// Request a GitHub Pages build
    /// You can request that your site be built from the latest revision on the default branch. This has the same effect as pushing a commit to your default branch, but does not require an additional commit. Manually triggering page builds can be helpful when diagnosing build warnings and failures.
    ///
    /// Build requests are limited to one concurrent build per repository and one concurrent build per requester. If you request a build while another is still in progress, the second request will be queued until the first completes.
    PostReposownerrepoPagesBuilds(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/builds/latest`
    /// * docs https://docs.github.com/rest/reference/repos#get-latest-pages-build
    ///
    /// Get latest Pages build
    ///
    GetReposownerrepoPagesBuildsLatest(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/builds/{build_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-github-pages-build
    ///
    /// Get GitHub Pages build
    ///
    GetReposownerrepoPagesBuildsbuildId(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/health`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-dns-health-check-for-github-pages
    ///
    /// Get a DNS health check for GitHub Pages
    /// Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.
    ///
    /// The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.
    ///
    /// Users must have admin or owner permissions. GitHub Apps must have the `pages:write` and `administration:write` permission to use this endpoint.
    GetReposownerrepoPagesHealth(String, String),
    /// * tags projects
    /// * get `/repos/{owner}/{repo}/projects`
    /// * docs https://docs.github.com/rest/reference/projects#list-repository-projects
    ///
    /// List repository projects
    /// Lists the projects in a repository. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    GetReposownerrepoProjects(String, String),
    /// * tags projects
    /// * post `/repos/{owner}/{repo}/projects`
    /// * docs https://docs.github.com/rest/reference/projects#create-a-repository-project
    ///
    /// Create a repository project
    /// Creates a repository project board. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    PostReposownerrepoProjects(String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls`
    /// * docs https://docs.github.com/rest/reference/pulls#list-pull-requests
    ///
    /// List pull requests
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetReposownerrepoPulls(String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls`
    /// * docs https://docs.github.com/rest/reference/pulls#create-a-pull-request
    ///
    /// Create a pull request
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
    ///
    /// You can create a new pull request.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-rate-limits)" for details.
    PostReposownerrepoPulls(String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/comments`
    /// * docs https://docs.github.com/rest/reference/pulls#list-review-comments-in-a-repository
    ///
    /// List review comments in a repository
    /// Lists review comments for all pull requests in a repository. By default, review comments are in ascending order by ID.
    GetReposownerrepoPullsComments(String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#get-a-review-comment-for-a-pull-request
    ///
    /// Get a review comment for a pull request
    /// Provides details for a review comment.
    GetReposownerrepoPullsCommentscommentId(String, String, String),
    /// * tags pulls
    /// * patch `/repos/{owner}/{repo}/pulls/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#update-a-review-comment-for-a-pull-request
    ///
    /// Update a review comment for a pull request
    /// Enables you to edit a review comment.
    PatchReposownerrepoPullsCommentscommentId(String, String, String),
    /// * tags pulls
    /// * delete `/repos/{owner}/{repo}/pulls/comments/{comment_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#delete-a-review-comment-for-a-pull-request
    ///
    /// Delete a review comment for a pull request
    /// Deletes a review comment.
    DeleteReposownerrepoPullsCommentscommentId(String, String, String),
    /// * tags reactions
    /// * get `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#list-reactions-for-a-pull-request-review-comment
    ///
    /// List reactions for a pull request review comment
    /// List the reactions to a [pull request review comment](https://docs.github.com/rest/reference/pulls#review-comments).
    GetReposownerrepoPullsCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions#create-reaction-for-a-pull-request-review-comment
    ///
    /// Create reaction for a pull request review comment
    /// Create a reaction to a [pull request review comment](https://docs.github.com/rest/reference/pulls#comments). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.
    PostReposownerrepoPullsCommentscommentIdReactions(String, String, String),
    /// * tags reactions
    /// * delete `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}`
    /// * docs https://docs.github.com/rest/reference/reactions#delete-a-pull-request-comment-reaction
    ///
    /// Delete a pull request comment reaction
    /// **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/pulls/comments/:comment_id/reactions/:reaction_id.`
    ///
    /// Delete a reaction to a [pull request review comment](https://docs.github.com/rest/reference/pulls#review-comments).
    DeleteReposownerrepoPullsCommentscommentIdReactionsreactionId(String, String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}`
    /// * docs https://docs.github.com/rest/reference/pulls#get-a-pull-request
    ///
    /// Get a pull request
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists details of a pull request by providing its number.
    ///
    /// When you get, [create](https://docs.github.com/rest/reference/pulls/#create-a-pull-request), or [edit](https://docs.github.com/rest/reference/pulls#update-a-pull-request) a pull request, GitHub creates a merge commit to test whether the pull request can be automatically merged into the base branch. This test commit is not added to the base branch or the head branch. You can review the status of the test commit using the `mergeable` key. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
    ///
    /// The value of the `mergeable` attribute can be `true`, `false`, or `null`. If the value is `null`, then GitHub has started a background job to compute the mergeability. After giving the job time to complete, resubmit the request. When the job finishes, you will see a non-`null` value for the `mergeable` attribute in the response. If `mergeable` is `true`, then `merge_commit_sha` will be the SHA of the _test_ merge commit.
    ///
    /// The value of the `merge_commit_sha` attribute changes depending on the state of the pull request. Before merging a pull request, the `merge_commit_sha` attribute holds the SHA of the _test_ merge commit. After merging a pull request, the `merge_commit_sha` attribute changes depending on how you merged the pull request:
    ///
    /// *   If merged as a [merge commit](https://help.github.com/articles/about-merge-methods-on-github/), `merge_commit_sha` represents the SHA of the merge commit.
    /// *   If merged via a [squash](https://help.github.com/articles/about-merge-methods-on-github/#squashing-your-merge-commits), `merge_commit_sha` represents the SHA of the squashed commit on the base branch.
    /// *   If [rebased](https://help.github.com/articles/about-merge-methods-on-github/#rebasing-and-merging-your-commits), `merge_commit_sha` represents the commit that the base branch was updated to.
    ///
    /// Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
    GetReposownerrepoPullspullNumber(String, String, String),
    /// * tags pulls
    /// * patch `/repos/{owner}/{repo}/pulls/{pull_number}`
    /// * docs https://docs.github.com/rest/reference/pulls/#update-a-pull-request
    ///
    /// Update a pull request
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
    PatchReposownerrepoPullspullNumber(String, String, String),
    /// * tags codespaces
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/codespaces`
    /// * docs https://docs.github.com/rest/reference/codespaces#create-a-codespace-from-a-pull-request
    ///
    /// Create a codespace from a pull request
    /// Creates a codespace owned by the authenticated user for the specified pull request.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PostReposownerrepoPullspullNumberCodespaces(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/comments`
    /// * docs https://docs.github.com/rest/reference/pulls#list-review-comments-on-a-pull-request
    ///
    /// List review comments on a pull request
    /// Lists all review comments for a pull request. By default, review comments are in ascending order by ID.
    GetReposownerrepoPullspullNumberComments(String, String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/comments`
    /// * docs https://docs.github.com/rest/reference/pulls#create-a-review-comment-for-a-pull-request
    ///
    /// Create a review comment for a pull request
    ///
    /// Creates a review comment in the pull request diff. To add a regular comment to a pull request timeline, see "[Create an issue comment](https://docs.github.com/rest/reference/issues#create-an-issue-comment)." We recommend creating a review comment using `line`, `side`, and optionally `start_line` and `start_side` if your comment applies to more than one line in the pull request diff.
    ///
    /// You can still create a review comment using the `position` parameter. When you use `position`, the `line`, `side`, `start_line`, and `start_side` parameters are not required.
    ///
    /// **Note:** The position value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoPullspullNumberComments(String, String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies`
    /// * docs https://docs.github.com/rest/reference/pulls#create-a-reply-for-a-review-comment
    ///
    /// Create a reply for a review comment
    /// Creates a reply to a review comment for a pull request. For the `comment_id`, provide the ID of the review comment you are replying to. This must be the ID of a _top-level review comment_, not a reply to that comment. Replies to replies are not supported.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoPullspullNumberCommentscommentIdReplies(String, String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/commits`
    /// * docs https://docs.github.com/rest/reference/pulls#list-commits-on-a-pull-request
    ///
    /// List commits on a pull request
    /// Lists a maximum of 250 commits for a pull request. To receive a complete commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/reference/repos#list-commits) endpoint.
    GetReposownerrepoPullspullNumberCommits(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/files`
    /// * docs https://docs.github.com/rest/reference/pulls#list-pull-requests-files
    ///
    /// List pull requests files
    /// **Note:** Responses include a maximum of 3000 files. The paginated response returns 30 files per page by default.
    GetReposownerrepoPullspullNumberFiles(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/merge`
    /// * docs https://docs.github.com/rest/reference/pulls#check-if-a-pull-request-has-been-merged
    ///
    /// Check if a pull request has been merged
    ///
    GetReposownerrepoPullspullNumberMerge(String, String, String),
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/merge`
    /// * docs https://docs.github.com/rest/reference/pulls#merge-a-pull-request
    ///
    /// Merge a pull request
    /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PutReposownerrepoPullspullNumberMerge(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`
    /// * docs https://docs.github.com/rest/reference/pulls#list-requested-reviewers-for-a-pull-request
    ///
    /// List requested reviewers for a pull request
    ///
    GetReposownerrepoPullspullNumberRequestedReviewers(String, String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`
    /// * docs https://docs.github.com/rest/reference/pulls#request-reviewers-for-a-pull-request
    ///
    /// Request reviewers for a pull request
    /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoPullspullNumberRequestedReviewers(String, String, String),
    /// * tags pulls
    /// * delete `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`
    /// * docs https://docs.github.com/rest/reference/pulls#remove-requested-reviewers-from-a-pull-request
    ///
    /// Remove requested reviewers from a pull request
    ///
    DeleteReposownerrepoPullspullNumberRequestedReviewers(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/reviews`
    /// * docs https://docs.github.com/rest/reference/pulls#list-reviews-for-a-pull-request
    ///
    /// List reviews for a pull request
    /// The list of reviews returns in chronological order.
    GetReposownerrepoPullspullNumberReviews(String, String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/reviews`
    /// * docs https://docs.github.com/rest/reference/pulls#create-a-review-for-a-pull-request
    ///
    /// Create a review for a pull request
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    ///
    /// Pull request reviews created in the `PENDING` state do not include the `submitted_at` property in the response.
    ///
    /// **Note:** To comment on a specific line in a file, you need to first determine the _position_ of that line in the diff. The GitHub REST API v3 offers the `application/vnd.github.v3.diff` [media type](https://docs.github.com/rest/overview/media-types#commits-commit-comparison-and-pull-requests). To see a pull request diff, add this media type to the `Accept` header of a call to the [single pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) endpoint.
    ///
    /// The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
    PostReposownerrepoPullspullNumberReviews(String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#get-a-review-for-a-pull-request
    ///
    /// Get a review for a pull request
    ///
    GetReposownerrepoPullspullNumberReviewsreviewId(String, String, String, String),
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#update-a-review-for-a-pull-request
    ///
    /// Update a review for a pull request
    /// Update the review summary comment with new text.
    PutReposownerrepoPullspullNumberReviewsreviewId(String, String, String, String),
    /// * tags pulls
    /// * delete `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs https://docs.github.com/rest/reference/pulls#delete-a-pending-review-for-a-pull-request
    ///
    /// Delete a pending review for a pull request
    ///
    DeleteReposownerrepoPullspullNumberReviewsreviewId(String, String, String, String),
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments`
    /// * docs https://docs.github.com/rest/reference/pulls#list-comments-for-a-pull-request-review
    ///
    /// List comments for a pull request review
    /// List comments for a specific pull request review.
    GetReposownerrepoPullspullNumberReviewsreviewIdComments(String, String, String, String),
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals`
    /// * docs https://docs.github.com/rest/reference/pulls#dismiss-a-review-for-a-pull-request
    ///
    /// Dismiss a review for a pull request
    /// **Note:** To dismiss a pull request review on a [protected branch](https://docs.github.com/rest/reference/repos#branches), you must be a repository administrator or be included in the list of people or teams who can dismiss pull request reviews.
    PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(String, String, String, String),
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events`
    /// * docs https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request
    ///
    /// Submit a review for a pull request
    ///
    PostReposownerrepoPullspullNumberReviewsreviewIdEvents(String, String, String, String),
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/update-branch`
    /// * docs https://docs.github.com/rest/reference/pulls#update-a-pull-request-branch
    ///
    /// Update a pull request branch
    /// Updates the pull request branch with the latest upstream changes by merging HEAD from the base branch into the pull request branch.
    PutReposownerrepoPullspullNumberUpdateBranch(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/readme`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-repository-readme
    ///
    /// Get a repository README
    /// Gets the preferred README for a repository.
    ///
    /// READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
    GetReposownerrepoReadme(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/readme/{dir}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-repository-directory-readme
    ///
    /// Get a repository README for a directory
    /// Gets the README from a repository directory.
    ///
    /// READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
    GetReposownerrepoReadmedir(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases`
    /// * docs https://docs.github.com/rest/reference/repos#list-releases
    ///
    /// List releases
    /// This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/reference/repos#list-repository-tags).
    ///
    /// Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
    GetReposownerrepoReleases(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/releases`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-release
    ///
    /// Create a release
    /// Users with push access to the repository can create a release.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostReposownerrepoReleases(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/assets/{asset_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-release-asset
    ///
    /// Get a release asset
    /// To download the asset's binary content, set the `Accept` header of the request to [`application/octet-stream`](https://docs.github.com/rest/overview/media-types). The API will either redirect the client to the location, or stream it directly if possible. API clients should handle both a `200` or `302` response.
    GetReposownerrepoReleasesAssetsassetId(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/releases/assets/{asset_id}`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-release-asset
    ///
    /// Update a release asset
    /// Users with push access to the repository can edit a release asset.
    PatchReposownerrepoReleasesAssetsassetId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/releases/assets/{asset_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-release-asset
    ///
    /// Delete a release asset
    ///
    DeleteReposownerrepoReleasesAssetsassetId(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/releases/generate-notes`
    /// * docs https://docs.github.com/rest/reference/repos#generate-release-notes
    ///
    /// Generate release notes content for a release
    /// Generate a name and body describing a [release](https://docs.github.com/rest/reference/repos#releases). The body content will be markdown formatted and contain information like the changes since last release and users who contributed. The generated release notes are not saved anywhere. They are intended to be generated and used when creating a new release.
    PostReposownerrepoReleasesGenerateNotes(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/latest`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-latest-release
    ///
    /// Get the latest release
    /// View the latest published full release for the repository.
    ///
    /// The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.
    GetReposownerrepoReleasesLatest(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/tags/{tag}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-release-by-tag-name
    ///
    /// Get a release by tag name
    /// Get a published release with the specified tag.
    GetReposownerrepoReleasesTagstag(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/{release_id}`
    /// * docs https://docs.github.com/rest/reference/repos#get-a-release
    ///
    /// Get a release
    /// **Note:** This returns an `upload_url` key corresponding to the endpoint for uploading release assets. This key is a [hypermedia resource](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia).
    GetReposownerrepoReleasesreleaseId(String, String, String),
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/releases/{release_id}`
    /// * docs https://docs.github.com/rest/reference/repos#update-a-release
    ///
    /// Update a release
    /// Users with push access to the repository can edit a release.
    PatchReposownerrepoReleasesreleaseId(String, String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/releases/{release_id}`
    /// * docs https://docs.github.com/rest/reference/repos#delete-a-release
    ///
    /// Delete a release
    /// Users with push access to the repository can delete a release.
    DeleteReposownerrepoReleasesreleaseId(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/{release_id}/assets`
    /// * docs https://docs.github.com/rest/reference/repos#list-release-assets
    ///
    /// List release assets
    ///
    GetReposownerrepoReleasesreleaseIdAssets(String, String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/releases/{release_id}/assets`
    /// * docs https://docs.github.com/rest/reference/repos#upload-a-release-asset
    ///
    /// Upload a release asset
    /// This endpoint makes use of [a Hypermedia relation](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia) to determine which URL to access. The endpoint you call to upload release assets is specific to your release. Use the `upload_url` returned in
    /// the response of the [Create a release endpoint](https://docs.github.com/rest/reference/repos#create-a-release) to upload a release asset.
    ///
    /// You need to use an HTTP client which supports [SNI](http://en.wikipedia.org/wiki/Server_Name_Indication) to make calls to this endpoint.
    ///
    /// Most libraries will set the required `Content-Length` header automatically. Use the required `Content-Type` header to provide the media type of the asset. For a list of media types, see [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml). For example:
    ///
    /// `application/zip`
    ///
    /// GitHub expects the asset data in its raw binary form, rather than JSON. You will send the raw binary content of the asset as the request body. Everything else about the endpoint is the same as the rest of the API. For example,
    /// you'll still need to pass your authentication to be able to upload an asset.
    ///
    /// When an upstream failure occurs, you will receive a `502 Bad Gateway` status. This may leave an empty asset with a state of `starter`. It can be safely deleted.
    ///
    /// **Notes:**
    /// *   GitHub renames asset filenames that have special characters, non-alphanumeric characters, and leading or trailing periods. The "[List assets for a release](https://docs.github.com/rest/reference/repos#list-assets-for-a-release)"
    /// endpoint lists the renamed filenames. For more information and help, contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
    /// *   If you upload an asset with the same filename as another uploaded asset, you'll receive an error and must delete the old file before you can re-upload the new asset.
    PostReposownerrepoReleasesreleaseIdAssets(String, String, String),
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/releases/{release_id}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-release
    ///
    /// Create reaction for a release
    /// Create a reaction to a [release](https://docs.github.com/rest/reference/repos#releases). A response with a `Status: 200 OK` means that you already added the reaction type to this release.
    PostReposownerrepoReleasesreleaseIdReactions(String, String, String),
    /// * tags secret-scanning
    /// * get `/repos/{owner}/{repo}/secret-scanning/alerts`
    /// * docs https://docs.github.com/rest/reference/secret-scanning#list-secret-scanning-alerts-for-a-repository
    ///
    /// List secret scanning alerts for a repository
    /// Lists secret scanning alerts for a private repository, from newest to oldest. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
    GetReposownerrepoSecretScanningAlerts(String, String),
    /// * tags secret-scanning
    /// * get `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}`
    /// * docs https://docs.github.com/rest/reference/secret-scanning#get-a-secret-scanning-alert
    ///
    /// Get a secret scanning alert
    /// Gets a single secret scanning alert detected in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
    GetReposownerrepoSecretScanningAlertsalertNumber(String, String, String),
    /// * tags secret-scanning
    /// * patch `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}`
    /// * docs https://docs.github.com/rest/reference/secret-scanning#update-a-secret-scanning-alert
    ///
    /// Update a secret scanning alert
    /// Updates the status of a secret scanning alert in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` write permission to use this endpoint.
    PatchReposownerrepoSecretScanningAlertsalertNumber(String, String, String),
    /// * tags secret-scanning
    /// * get `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations`
    /// * docs https://docs.github.com/rest/reference/secret-scanning#list-locations-for-a-secret-scanning-alert
    ///
    /// List locations for a secret scanning alert
    /// Lists all locations for a given secret scanning alert for a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
    GetReposownerrepoSecretScanningAlertsalertNumberLocations(String, String, String),
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/stargazers`
    /// * docs https://docs.github.com/rest/reference/activity#list-stargazers
    ///
    /// List stargazers
    /// Lists the people that have starred the repository.
    ///
    /// You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
    GetReposownerrepoStargazers(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/code_frequency`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-weekly-commit-activity
    ///
    /// Get the weekly commit activity
    /// Returns a weekly aggregate of the number of additions and deletions pushed to a repository.
    GetReposownerrepoStatsCodeFrequency(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/commit_activity`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-last-year-of-commit-activity
    ///
    /// Get the last year of commit activity
    /// Returns the last year of commit activity grouped by week. The `days` array is a group of commits per day, starting on `Sunday`.
    GetReposownerrepoStatsCommitActivity(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/contributors`
    /// * docs https://docs.github.com/rest/reference/repos#get-all-contributor-commit-activity
    ///
    /// Get all contributor commit activity
    ///
    /// Returns the `total` number of commits authored by the contributor. In addition, the response includes a Weekly Hash (`weeks` array) with the following information:
    ///
    /// *   `w` - Start of the week, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
    /// *   `a` - Number of additions
    /// *   `d` - Number of deletions
    /// *   `c` - Number of commits
    GetReposownerrepoStatsContributors(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/participation`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-weekly-commit-count
    ///
    /// Get the weekly commit count
    /// Returns the total commit counts for the `owner` and total commit counts in `all`. `all` is everyone combined, including the `owner` in the last 52 weeks. If you'd like to get the commit counts for non-owners, you can subtract `owner` from `all`.
    ///
    /// The array order is oldest week (index 0) to most recent week.
    GetReposownerrepoStatsParticipation(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/punch_card`
    /// * docs https://docs.github.com/rest/reference/repos#get-the-hourly-commit-count-for-each-day
    ///
    /// Get the hourly commit count for each day
    /// Each array contains the day number, hour number, and number of commits:
    ///
    /// *   `0-6`: Sunday - Saturday
    /// *   `0-23`: Hour of day
    /// *   Number of commits
    ///
    /// For example, `[2, 14, 25]` indicates that there were 25 total commits, during the 2:00pm hour on Tuesdays. All times are based on the time zone of individual commits.
    GetReposownerrepoStatsPunchCard(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/statuses/{sha}`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-commit-status
    ///
    /// Create a commit status
    /// Users with push access in a repository can create commit statuses for a given SHA.
    ///
    /// Note: there is a limit of 1000 statuses per `sha` and `context` within a repository. Attempts to create more than 1000 statuses will result in a validation error.
    PostReposownerrepoStatusessha(String, String, String),
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/subscribers`
    /// * docs https://docs.github.com/rest/reference/activity#list-watchers
    ///
    /// List watchers
    /// Lists the people watching the specified repository.
    GetReposownerrepoSubscribers(String, String),
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#get-a-repository-subscription
    ///
    /// Get a repository subscription
    ///
    GetReposownerrepoSubscription(String, String),
    /// * tags activity
    /// * put `/repos/{owner}/{repo}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#set-a-repository-subscription
    ///
    /// Set a repository subscription
    /// If you would like to watch a repository, set `subscribed` to `true`. If you would like to ignore notifications made within a repository, set `ignored` to `true`. If you would like to stop watching a repository, [delete the repository's subscription](https://docs.github.com/rest/reference/activity#delete-a-repository-subscription) completely.
    PutReposownerrepoSubscription(String, String),
    /// * tags activity
    /// * delete `/repos/{owner}/{repo}/subscription`
    /// * docs https://docs.github.com/rest/reference/activity#delete-a-repository-subscription
    ///
    /// Delete a repository subscription
    /// This endpoint should only be used to stop watching a repository. To control whether or not you wish to receive notifications from a repository, [set the repository's subscription manually](https://docs.github.com/rest/reference/activity#set-a-repository-subscription).
    DeleteReposownerrepoSubscription(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/tags`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-tags
    ///
    /// List repository tags
    ///
    GetReposownerrepoTags(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/tarball/{ref}`
    /// * docs https://docs.github.com/rest/reference/repos#download-a-repository-archive
    ///
    /// Download a repository archive (tar)
    /// Gets a redirect URL to download a tar archive for a repository. If you omit `:ref`, the repository’s default branch (usually
    /// `master`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
    /// the `Location` header to make a second `GET` request.
    /// **Note**: For private repositories, these links are temporary and expire after five minutes.
    GetReposownerrepoTarballref(String, String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/teams`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-teams
    ///
    /// List repository teams
    ///
    GetReposownerrepoTeams(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/topics`
    /// * docs https://docs.github.com/rest/reference/repos#get-all-repository-topics
    ///
    /// Get all repository topics
    ///
    GetReposownerrepoTopics(String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/topics`
    /// * docs https://docs.github.com/rest/reference/repos#replace-all-repository-topics
    ///
    /// Replace all repository topics
    ///
    PutReposownerrepoTopics(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/clones`
    /// * docs https://docs.github.com/rest/reference/repos#get-repository-clones
    ///
    /// Get repository clones
    /// Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
    GetReposownerrepoTrafficClones(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/popular/paths`
    /// * docs https://docs.github.com/rest/reference/repos#get-top-referral-paths
    ///
    /// Get top referral paths
    /// Get the top 10 popular contents over the last 14 days.
    GetReposownerrepoTrafficPopularPaths(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/popular/referrers`
    /// * docs https://docs.github.com/rest/reference/repos#get-top-referral-sources
    ///
    /// Get top referral sources
    /// Get the top 10 referrers over the last 14 days.
    GetReposownerrepoTrafficPopularReferrers(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/views`
    /// * docs https://docs.github.com/rest/reference/repos#get-page-views
    ///
    /// Get page views
    /// Get the total number of views and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
    GetReposownerrepoTrafficViews(String, String),
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/transfer`
    /// * docs https://docs.github.com/rest/reference/repos#transfer-a-repository
    ///
    /// Transfer a repository
    /// A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://help.github.com/articles/about-repository-transfers/).
    PostReposownerrepoTransfer(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/vulnerability-alerts`
    /// * docs https://docs.github.com/rest/reference/repos#check-if-vulnerability-alerts-are-enabled-for-a-repository
    ///
    /// Check if vulnerability alerts are enabled for a repository
    /// Shows whether dependency alerts are enabled or disabled for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
    GetReposownerrepoVulnerabilityAlerts(String, String),
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/vulnerability-alerts`
    /// * docs https://docs.github.com/rest/reference/repos#enable-vulnerability-alerts
    ///
    /// Enable vulnerability alerts
    /// Enables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
    PutReposownerrepoVulnerabilityAlerts(String, String),
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/vulnerability-alerts`
    /// * docs https://docs.github.com/rest/reference/repos#disable-vulnerability-alerts
    ///
    /// Disable vulnerability alerts
    /// Disables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
    DeleteReposownerrepoVulnerabilityAlerts(String, String),
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/zipball/{ref}`
    /// * docs https://docs.github.com/rest/reference/repos#download-a-repository-archive
    ///
    /// Download a repository archive (zip)
    /// Gets a redirect URL to download a zip archive for a repository. If you omit `:ref`, the repository’s default branch (usually
    /// `master`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
    /// the `Location` header to make a second `GET` request.
    /// **Note**: For private repositories, these links are temporary and expire after five minutes.
    GetReposownerrepoZipballref(String, String, String),
    /// * tags repos
    /// * post `/repos/{template_owner}/{template_repo}/generate`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-repository-using-a-template
    ///
    /// Create a repository using a template
    /// Creates a new repository using a repository template. Use the `template_owner` and `template_repo` route parameters to specify the repository to use as the template. The authenticated user must own or be a member of an organization that owns the repository. To check if a repository is available to use as a template, get the repository's information using the [Get a repository](https://docs.github.com/rest/reference/repos#get-a-repository) endpoint and check that the `is_template` key is `true`.
    ///
    /// **OAuth scope requirements**
    ///
    /// When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
    ///
    /// *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
    /// *   `repo` scope to create a private repository
    PostRepostemplateOwnertemplateRepoGenerate(String, String),
    /// * tags repos
    /// * get `/repositories`
    /// * docs https://docs.github.com/rest/reference/repos#list-public-repositories
    ///
    /// List public repositories
    /// Lists all public repositories in the order that they were created.
    ///
    /// Note:
    /// - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise.
    /// - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of repositories.
    GetRepositories(),
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets`
    /// * docs https://docs.github.com/rest/reference/actions#list-environment-secrets
    ///
    /// List environment secrets
    /// Lists all secrets available in an environment without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(String, String),
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets/public-key`
    /// * docs https://docs.github.com/rest/reference/actions#get-an-environment-public-key
    ///
    /// Get an environment public key
    /// Get the public key for an environment, which you need to encrypt environment secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(String, String),
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#get-an-environment-secret
    ///
    /// Get an environment secret
    /// Gets a single environment secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(String, String, String),
    /// * tags actions
    /// * put `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#create-or-update-an-environment-secret
    ///
    /// Create or update an environment secret
    /// Creates or updates an environment secret with an encrypted value. Encrypt your secret using
    /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
    /// token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use
    /// this endpoint.
    ///
    /// #### Example encrypting a secret using Node.js
    ///
    /// Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
    ///
    /// ```
    /// const sodium = require('tweetsodium');
    ///
    /// const key = "base64-encoded-public-key";
    /// const value = "plain-text-secret";
    ///
    /// // Convert the message and key to Uint8Array's (Buffer implements that interface)
    /// const messageBytes = Buffer.from(value);
    /// const keyBytes = Buffer.from(key, 'base64');
    ///
    /// // Encrypt using LibSodium.
    /// const encryptedBytes = sodium.seal(messageBytes, keyBytes);
    ///
    /// // Base64 the encrypted secret
    /// const encrypted = Buffer.from(encryptedBytes).toString('base64');
    ///
    /// console.log(encrypted);
    /// ```
    ///
    ///
    /// #### Example encrypting a secret using Python
    ///
    /// Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
    ///
    /// ```
    /// from base64 import b64encode
    /// from nacl import encoding, public
    ///
    /// def encrypt(public_key: str, secret_value: str) -> str:
    ///   """Encrypt a Unicode string using the public key."""
    ///   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
    ///   sealed_box = public.SealedBox(public_key)
    ///   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
    ///   return b64encode(encrypted).decode("utf-8")
    /// ```
    ///
    /// #### Example encrypting a secret using C#
    ///
    /// Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
    ///
    /// ```
    /// var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
    /// var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
    ///
    /// var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
    ///
    /// Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
    /// ```
    ///
    /// #### Example encrypting a secret using Ruby
    ///
    /// Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
    ///
    /// ```ruby
    /// require "rbnacl"
    /// require "base64"
    ///
    /// key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
    /// public_key = RbNaCl::PublicKey.new(key)
    ///
    /// box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
    /// encrypted_secret = box.encrypt("my_secret")
    ///
    /// # Print the base64 encoded secret
    /// puts Base64.strict_encode64(encrypted_secret)
    /// ```
    PutRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(String, String, String),
    /// * tags actions
    /// * delete `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/actions#delete-an-environment-secret
    ///
    /// Delete an environment secret
    /// Deletes a secret in an environment using the secret name. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    DeleteRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(
        String,
        String,
        String,
    ),
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Groups`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-provisioned-scim-groups-for-an-enterprise
    ///
    /// List provisioned SCIM groups for an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    GetScimV2EnterprisesenterpriseGroups(String),
    /// * tags enterprise-admin
    /// * post `/scim/v2/enterprises/{enterprise}/Groups`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#provision-a-scim-enterprise-group-and-invite-users
    ///
    /// Provision a SCIM enterprise group and invite users
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Provision an enterprise group, and invite users to the group. This sends invitation emails to the email address of the invited users to join the GitHub organization that the SCIM group corresponds to.
    PostScimV2EnterprisesenterpriseGroups(String),
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-group
    ///
    /// Get SCIM provisioning information for an enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    GetScimV2EnterprisesenterpriseGroupsscimGroupId(String, String),
    /// * tags enterprise-admin
    /// * put `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-group
    ///
    /// Set SCIM information for a provisioned enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Replaces an existing provisioned group’s information. You must provide all the information required for the group as if you were provisioning it for the first time. Any existing group information that you don't provide will be removed, including group membership. If you want to only update a specific attribute, use the [Update an attribute for a SCIM enterprise group](#update-an-attribute-for-a-scim-enterprise-group) endpoint instead.
    PutScimV2EnterprisesenterpriseGroupsscimGroupId(String, String),
    /// * tags enterprise-admin
    /// * patch `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-group
    ///
    /// Update an attribute for a SCIM enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Allows you to change a provisioned group’s individual attributes. To change a group’s values, you must provide a specific Operations JSON format that contains at least one of the add, remove, or replace operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    PatchScimV2EnterprisesenterpriseGroupsscimGroupId(String, String),
    /// * tags enterprise-admin
    /// * delete `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#delete-a-scim-group-from-an-enterprise
    ///
    /// Delete a SCIM group from an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    DeleteScimV2EnterprisesenterpriseGroupsscimGroupId(String, String),
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Users`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#list-scim-provisioned-identities-for-an-enterprise
    ///
    /// List SCIM provisioned identities for an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Retrieves a paginated list of all provisioned enterprise members, including pending invitations.
    ///
    /// When a user with a SAML-provisioned external identity leaves (or is removed from) an enterprise, the account's metadata is immediately removed. However, the returned list of user accounts might not always match the organization or enterprise member list you see on GitHub. This can happen in certain cases where an external identity associated with an organization will not match an organization member:
    ///   - When a user with a SCIM-provisioned external identity is removed from an enterprise, the account's metadata is preserved to allow the user to re-join the organization in the future.
    ///   - When inviting a user to join an organization, you can expect to see their external identity in the results before they accept the invitation, or if the invitation is cancelled (or never accepted).
    ///   - When a user is invited over SCIM, an external identity is created that matches with the invitee's email address. However, this identity is only linked to a user account when the user accepts the invitation by going through SAML SSO.
    ///
    /// The returned list of external identities can include an entry for a `null` user. These are unlinked SAML identities that are created when a user goes through the following Single Sign-On (SSO) process but does not sign in to their GitHub account after completing SSO:
    ///
    /// 1. The user is granted access by the IdP and is not a member of the GitHub enterprise.
    ///
    /// 1. The user attempts to access the GitHub enterprise and initiates the SAML SSO process, and is not currently signed in to their GitHub account.
    ///
    /// 1. After successfully authenticating with the SAML SSO IdP, the `null` external identity entry is created and the user is prompted to sign in to their GitHub account:
    ///    - If the user signs in, their GitHub account is linked to this entry.
    ///    - If the user does not sign in (or does not create a new account when prompted), they are not added to the GitHub enterprise, and the external identity `null` entry remains in place.
    GetScimV2EnterprisesenterpriseUsers(String),
    /// * tags enterprise-admin
    /// * post `/scim/v2/enterprises/{enterprise}/Users`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#provision-and-invite-a-scim-enterprise-user
    ///
    /// Provision and invite a SCIM enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Provision enterprise membership for a user, and send organization invitation emails to the email address.
    ///
    /// You can optionally include the groups a user will be invited to join. If you do not provide a list of `groups`, the user is provisioned for the enterprise, but no organization invitation emails will be sent.
    PostScimV2EnterprisesenterpriseUsers(String),
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-user
    ///
    /// Get SCIM provisioning information for an enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    GetScimV2EnterprisesenterpriseUsersscimUserId(String, String),
    /// * tags enterprise-admin
    /// * put `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-user
    ///
    /// Set SCIM information for a provisioned enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](#update-an-attribute-for-an-enterprise-scim-user) endpoint instead.
    ///
    /// You must at least provide the required values for the user: `userName`, `name`, and `emails`.
    ///
    /// **Warning:** Setting `active: false` removes the user from the enterprise, deletes the external identity, and deletes the associated `{scim_user_id}`.
    PutScimV2EnterprisesenterpriseUsersscimUserId(String, String),
    /// * tags enterprise-admin
    /// * patch `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-user
    ///
    /// Update an attribute for a SCIM enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    ///
    /// **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
    ///
    /// **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the enterprise, deletes the external identity, and deletes the associated `:scim_user_id`.
    ///
    /// ```
    /// {
    ///   "Operations":[{
    ///     "op":"replace",
    ///     "value":{
    ///       "active":false
    ///     }
    ///   }]
    /// }
    /// ```
    PatchScimV2EnterprisesenterpriseUsersscimUserId(String, String),
    /// * tags enterprise-admin
    /// * delete `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/enterprise-admin#delete-a-scim-user-from-an-enterprise
    ///
    /// Delete a SCIM user from an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    DeleteScimV2EnterprisesenterpriseUsersscimUserId(String, String),
    /// * tags scim
    /// * get `/scim/v2/organizations/{org}/Users`
    /// * docs https://docs.github.com/rest/reference/scim#list-scim-provisioned-identities
    ///
    /// List SCIM provisioned identities
    /// Retrieves a paginated list of all provisioned organization members, including pending invitations. If you provide the `filter` parameter, the resources for all matching provisions members are returned.
    ///
    /// When a user with a SAML-provisioned external identity leaves (or is removed from) an organization, the account's metadata is immediately removed. However, the returned list of user accounts might not always match the organization or enterprise member list you see on GitHub. This can happen in certain cases where an external identity associated with an organization will not match an organization member:
    ///   - When a user with a SCIM-provisioned external identity is removed from an organization, the account's metadata is preserved to allow the user to re-join the organization in the future.
    ///   - When inviting a user to join an organization, you can expect to see their external identity in the results before they accept the invitation, or if the invitation is cancelled (or never accepted).
    ///   - When a user is invited over SCIM, an external identity is created that matches with the invitee's email address. However, this identity is only linked to a user account when the user accepts the invitation by going through SAML SSO.
    ///
    /// The returned list of external identities can include an entry for a `null` user. These are unlinked SAML identities that are created when a user goes through the following Single Sign-On (SSO) process but does not sign in to their GitHub account after completing SSO:
    ///
    /// 1. The user is granted access by the IdP and is not a member of the GitHub organization.
    ///
    /// 1. The user attempts to access the GitHub organization and initiates the SAML SSO process, and is not currently signed in to their GitHub account.
    ///
    /// 1. After successfully authenticating with the SAML SSO IdP, the `null` external identity entry is created and the user is prompted to sign in to their GitHub account:
    ///    - If the user signs in, their GitHub account is linked to this entry.
    ///    - If the user does not sign in (or does not create a new account when prompted), they are not added to the GitHub organization, and the external identity `null` entry remains in place.
    GetScimV2OrganizationsorgUsers(String),
    /// * tags scim
    /// * post `/scim/v2/organizations/{org}/Users`
    /// * docs https://docs.github.com/rest/reference/scim#provision-and-invite-a-scim-user
    ///
    /// Provision and invite a SCIM user
    /// Provision organization membership for a user, and send an activation email to the email address.
    PostScimV2OrganizationsorgUsers(String),
    /// * tags scim
    /// * get `/scim/v2/organizations/{org}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/scim#get-scim-provisioning-information-for-a-user
    ///
    /// Get SCIM provisioning information for a user
    ///
    GetScimV2OrganizationsorgUsersscimUserId(String, String),
    /// * tags scim
    /// * put `/scim/v2/organizations/{org}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/scim#set-scim-information-for-a-provisioned-user
    ///
    /// Update a provisioned organization membership
    /// Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user) endpoint instead.
    ///
    /// You must at least provide the required values for the user: `userName`, `name`, and `emails`.
    ///
    /// **Warning:** Setting `active: false` removes the user from the organization, deletes the external identity, and deletes the associated `{scim_user_id}`.
    PutScimV2OrganizationsorgUsersscimUserId(String, String),
    /// * tags scim
    /// * patch `/scim/v2/organizations/{org}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user
    ///
    /// Update an attribute for a SCIM user
    /// Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    ///
    /// **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
    ///
    /// **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the organization, deletes the external identity, and deletes the associated `:scim_user_id`.
    ///
    /// ```
    /// {
    ///   "Operations":[{
    ///     "op":"replace",
    ///     "value":{
    ///       "active":false
    ///     }
    ///   }]
    /// }
    /// ```
    PatchScimV2OrganizationsorgUsersscimUserId(String, String),
    /// * tags scim
    /// * delete `/scim/v2/organizations/{org}/Users/{scim_user_id}`
    /// * docs https://docs.github.com/rest/reference/scim#delete-a-scim-user-from-an-organization
    ///
    /// Delete a SCIM user from an organization
    ///
    DeleteScimV2OrganizationsorgUsersscimUserId(String, String),
    /// * tags search
    /// * get `/search/code`
    /// * docs https://docs.github.com/rest/reference/search#search-code
    ///
    /// Search code
    /// Searches for query terms inside of a file. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for code, you can get text match metadata for the file **content** and file **path** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find the definition of the `addClass` function inside [jQuery](https://github.com/jquery/jquery) repository, your query would look something like this:
    ///
    /// `q=addClass+in:file+language:js+repo:jquery/jquery`
    ///
    /// This query searches for the keyword `addClass` within a file's contents. The query limits the search to files where the language is JavaScript in the `jquery/jquery` repository.
    ///
    /// #### Considerations for code search
    ///
    /// Due to the complexity of searching code, there are a few restrictions on how searches are performed:
    ///
    /// *   Only the _default branch_ is considered. In most cases, this will be the `master` branch.
    /// *   Only files smaller than 384 KB are searchable.
    /// *   You must always include at least one search term when searching source code. For example, searching for [`language:go`](https://github.com/search?utf8=%E2%9C%93&q=language%3Ago&type=Code) is not valid, while [`amazing
    /// language:go`](https://github.com/search?utf8=%E2%9C%93&q=amazing+language%3Ago&type=Code) is.
    GetSearchCode(),
    /// * tags search
    /// * get `/search/commits`
    /// * docs https://docs.github.com/rest/reference/search#search-commits
    ///
    /// Search commits
    /// Find commits via various criteria on the default branch (usually `master`). This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for commits, you can get text match metadata for the **message** field when you provide the `text-match` media type. For more details about how to receive highlighted search results, see [Text match
    /// metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find commits related to CSS in the [octocat/Spoon-Knife](https://github.com/octocat/Spoon-Knife) repository. Your query would look something like this:
    ///
    /// `q=repo:octocat/Spoon-Knife+css`
    GetSearchCommits(),
    /// * tags search
    /// * get `/search/issues`
    /// * docs https://docs.github.com/rest/reference/search#search-issues-and-pull-requests
    ///
    /// Search issues and pull requests
    /// Find issues by state and keyword. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for issues, you can get text match metadata for the issue **title**, issue **body**, and issue **comment body** fields when you pass the `text-match` media type. For more details about how to receive highlighted
    /// search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find the oldest unresolved Python bugs on Windows. Your query might look something like this.
    ///
    /// `q=windows+label:bug+language:python+state:open&sort=created&order=asc`
    ///
    /// This query searches for the keyword `windows`, within any open issue that is labeled as `bug`. The search runs across repositories whose primary language is Python. The results are sorted by creation date in ascending order, which means the oldest issues appear first in the search results.
    ///
    /// **Note:** For [user-to-server](https://docs.github.com/developers/apps/identifying-and-authorizing-users-for-github-apps#user-to-server-requests) GitHub App requests, you can't retrieve a combination of issues and pull requests in a single query. Requests that don't include the `is:issue` or `is:pull-request` qualifier will receive an HTTP `422 Unprocessable Entity` response. To get results for both issues and pull requests, you must send separate queries for issues and pull requests. For more information about the `is` qualifier, see "[Searching only issues or pull requests](https://docs.github.com/github/searching-for-information-on-github/searching-issues-and-pull-requests#search-only-issues-or-pull-requests)."
    GetSearchIssues(),
    /// * tags search
    /// * get `/search/labels`
    /// * docs https://docs.github.com/rest/reference/search#search-labels
    ///
    /// Search labels
    /// Find labels in a repository with names or descriptions that match search keywords. Returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for labels, you can get text match metadata for the label **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find labels in the `linguist` repository that match `bug`, `defect`, or `enhancement`. Your query might look like this:
    ///
    /// `q=bug+defect+enhancement&repository_id=64778136`
    ///
    /// The labels that best match the query appear first in the search results.
    GetSearchLabels(),
    /// * tags search
    /// * get `/search/repositories`
    /// * docs https://docs.github.com/rest/reference/search#search-repositories
    ///
    /// Search repositories
    /// Find repositories via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for repositories, you can get text match metadata for the **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to search for popular Tetris repositories written in assembly code, your query might look like this:
    ///
    /// `q=tetris+language:assembly&sort=stars&order=desc`
    ///
    /// This query searches for repositories with the word `tetris` in the name, the description, or the README. The results are limited to repositories where the primary language is assembly. The results are sorted by stars in descending order, so that the most popular repositories appear first in the search results.
    GetSearchRepositories(),
    /// * tags search
    /// * get `/search/topics`
    /// * docs https://docs.github.com/rest/reference/search#search-topics
    ///
    /// Search topics
    /// Find topics via various criteria. Results are sorted by best match. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination). See "[Searching topics](https://help.github.com/articles/searching-topics/)" for a detailed list of qualifiers.
    ///
    /// When searching for topics, you can get text match metadata for the topic's **short\_description**, **description**, **name**, or **display\_name** field when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to search for topics related to Ruby that are featured on https://github.com/topics. Your query might look like this:
    ///
    /// `q=ruby+is:featured`
    ///
    /// This query searches for topics with the keyword `ruby` and limits the results to find only topics that are featured. The topics that are the best match for the query appear first in the search results.
    GetSearchTopics(),
    /// * tags search
    /// * get `/search/users`
    /// * docs https://docs.github.com/rest/reference/search#search-users
    ///
    /// Search users
    /// Find users via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for users, you can get text match metadata for the issue **login**, **email**, and **name** fields when you pass the `text-match` media type. For more details about highlighting search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata). For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you're looking for a list of popular users, you might try this query:
    ///
    /// `q=tom+repos:%3E42+followers:%3E1000`
    ///
    /// This query searches for users with the name `tom`. The results are restricted to users with more than 42 repositories and over 1,000 followers.
    GetSearchUsers(),
    /// * tags teams
    /// * get `/teams/{team_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#get-a-team-legacy
    ///
    /// Get a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/reference/teams#get-a-team-by-name) endpoint.
    GetTeamsteamId(String),
    /// * tags teams
    /// * patch `/teams/{team_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#update-a-team-legacy
    ///
    /// Update a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/reference/teams#update-a-team) endpoint.
    ///
    /// To edit a team, the authenticated user must either be an organization owner or a team maintainer.
    ///
    /// **Note:** With nested teams, the `privacy` for parent teams cannot be `secret`.
    PatchTeamsteamId(String),
    /// * tags teams
    /// * delete `/teams/{team_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#delete-a-team-legacy
    ///
    /// Delete a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a team](https://docs.github.com/rest/reference/teams#delete-a-team) endpoint.
    ///
    /// To delete a team, the authenticated user must be an organization owner or team maintainer.
    ///
    /// If you are an organization owner, deleting a parent team will delete all of its child teams as well.
    DeleteTeamsteamId(String),
    /// * tags teams
    /// * get `/teams/{team_id}/discussions`
    /// * docs https://docs.github.com/rest/reference/teams#list-discussions-legacy
    ///
    /// List discussions (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List discussions`](https://docs.github.com/rest/reference/teams#list-discussions) endpoint.
    ///
    /// List all discussions on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussions(String),
    /// * tags teams
    /// * post `/teams/{team_id}/discussions`
    /// * docs https://docs.github.com/rest/reference/teams#create-a-discussion-legacy
    ///
    /// Create a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create a discussion`](https://docs.github.com/rest/reference/teams#create-a-discussion) endpoint.
    ///
    /// Creates a new discussion post on a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostTeamsteamIdDiscussions(String),
    /// * tags teams
    /// * get `/teams/{team_id}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#get-a-discussion-legacy
    ///
    /// Get a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion](https://docs.github.com/rest/reference/teams#get-a-discussion) endpoint.
    ///
    /// Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussionsdiscussionNumber(String, String),
    /// * tags teams
    /// * patch `/teams/{team_id}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#update-a-discussion-legacy
    ///
    /// Update a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion](https://docs.github.com/rest/reference/teams#update-a-discussion) endpoint.
    ///
    /// Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    PatchTeamsteamIdDiscussionsdiscussionNumber(String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/discussions/{discussion_number}`
    /// * docs https://docs.github.com/rest/reference/teams#delete-a-discussion-legacy
    ///
    /// Delete a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Delete a discussion`](https://docs.github.com/rest/reference/teams#delete-a-discussion) endpoint.
    ///
    /// Delete a discussion from a team's page. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteTeamsteamIdDiscussionsdiscussionNumber(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/discussions/{discussion_number}/comments`
    /// * docs https://docs.github.com/rest/reference/teams#list-discussion-comments-legacy
    ///
    /// List discussion comments (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List discussion comments](https://docs.github.com/rest/reference/teams#list-discussion-comments) endpoint.
    ///
    /// List all comments on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussionsdiscussionNumberComments(String, String),
    /// * tags teams
    /// * post `/teams/{team_id}/discussions/{discussion_number}/comments`
    /// * docs https://docs.github.com/rest/reference/teams#create-a-discussion-comment-legacy
    ///
    /// Create a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Create a discussion comment](https://docs.github.com/rest/reference/teams#create-a-discussion-comment) endpoint.
    ///
    /// Creates a new comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    PostTeamsteamIdDiscussionsdiscussionNumberComments(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#get-a-discussion-comment-legacy
    ///
    /// Get a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion comment](https://docs.github.com/rest/reference/teams#get-a-discussion-comment) endpoint.
    ///
    /// Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(String, String, String),
    /// * tags teams
    /// * patch `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#update-a-discussion-comment-legacy
    ///
    /// Update a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion comment](https://docs.github.com/rest/reference/teams#update-a-discussion-comment) endpoint.
    ///
    /// Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(String, String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs https://docs.github.com/rest/reference/teams#delete-a-discussion-comment-legacy
    ///
    /// Delete a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a discussion comment](https://docs.github.com/rest/reference/teams#delete-a-discussion-comment) endpoint.
    ///
    /// Deletes a comment on a team discussion. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(String, String, String),
    /// * tags reactions
    /// * get `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-comment-legacy
    ///
    /// List reactions for a team discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion comment`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment) endpoint.
    ///
    /// List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(String, String, String),
    /// * tags reactions
    /// * post `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-team-discussion-comment-legacy
    ///
    /// Create reaction for a team discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Create reaction for a team discussion comment](https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion-comment)" endpoint.
    ///
    /// Create a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
    PostTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(
        String,
        String,
        String,
    ),
    /// * tags reactions
    /// * get `/teams/{team_id}/discussions/{discussion_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-legacy
    ///
    /// List reactions for a team discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion) endpoint.
    ///
    /// List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetTeamsteamIdDiscussionsdiscussionNumberReactions(String, String),
    /// * tags reactions
    /// * post `/teams/{team_id}/discussions/{discussion_number}/reactions`
    /// * docs https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-team-discussion-legacy
    ///
    /// Create reaction for a team discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create reaction for a team discussion`](https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion) endpoint.
    ///
    /// Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
    PostTeamsteamIdDiscussionsdiscussionNumberReactions(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/invitations`
    /// * docs https://docs.github.com/rest/reference/teams#list-pending-team-invitations-legacy
    ///
    /// List pending team invitations (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/reference/teams#list-pending-team-invitations) endpoint.
    ///
    /// The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.
    GetTeamsteamIdInvitations(String),
    /// * tags teams
    /// * get `/teams/{team_id}/members`
    /// * docs https://docs.github.com/rest/reference/teams#list-team-members-legacy
    ///
    /// List team members (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/reference/teams#list-team-members) endpoint.
    ///
    /// Team members will include the members of child teams.
    GetTeamsteamIdMembers(String),
    /// * tags teams
    /// * get `/teams/{team_id}/members/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#get-team-member-legacy
    ///
    /// Get team member (Legacy)
    /// The "Get team member" endpoint (described below) is deprecated.
    ///
    /// We recommend using the [Get team membership for a user](https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user) endpoint instead. It allows you to get both active and pending memberships.
    ///
    /// To list members in a team, the team must be visible to the authenticated user.
    GetTeamsteamIdMembersusername(String, String),
    /// * tags teams
    /// * put `/teams/{team_id}/members/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#add-team-member-legacy
    ///
    /// Add team member (Legacy)
    /// The "Add team member" endpoint (described below) is deprecated.
    ///
    /// We recommend using the [Add or update team membership for a user](https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user) endpoint instead. It allows you to invite new organization members to your teams.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To add someone to a team, the authenticated user must be an organization owner or a team maintainer in the team they're changing. The person being added to the team must be a member of the team's organization.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutTeamsteamIdMembersusername(String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/members/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#remove-team-member-legacy
    ///
    /// Remove team member (Legacy)
    /// The "Remove team member" endpoint (described below) is deprecated.
    ///
    /// We recommend using the [Remove team membership for a user](https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user) endpoint instead. It allows you to remove both active and pending memberships.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To remove a team member, the authenticated user must have 'admin' permissions to the team or be an owner of the org that the team is associated with. Removing a team member does not delete the user, it just removes them from the team.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    DeleteTeamsteamIdMembersusername(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user-legacy
    ///
    /// Get team membership for a user (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user) endpoint.
    ///
    /// Team members will include the members of child teams.
    ///
    /// To get a user's membership with a team, the team must be visible to the authenticated user.
    ///
    /// **Note:**
    /// The response contains the `state` of the membership and the member's `role`.
    ///
    /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
    GetTeamsteamIdMembershipsusername(String, String),
    /// * tags teams
    /// * put `/teams/{team_id}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user-legacy
    ///
    /// Add or update team membership for a user (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the "pending" state until the user accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.
    ///
    /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
    PutTeamsteamIdMembershipsusername(String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/memberships/{username}`
    /// * docs https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user-legacy
    ///
    /// Remove team membership for a user (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove team membership for a user](https://docs.github.com/rest/reference/teams#remove-team-membership-for-a-user) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    DeleteTeamsteamIdMembershipsusername(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/projects`
    /// * docs https://docs.github.com/rest/reference/teams/#list-team-projects-legacy
    ///
    /// List team projects (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team projects`](https://docs.github.com/rest/reference/teams#list-team-projects) endpoint.
    ///
    /// Lists the organization projects for a team.
    GetTeamsteamIdProjects(String),
    /// * tags teams
    /// * get `/teams/{team_id}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-project-legacy
    ///
    /// Check team permissions for a project (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a project](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project) endpoint.
    ///
    /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
    GetTeamsteamIdProjectsprojectId(String, String),
    /// * tags teams
    /// * put `/teams/{team_id}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#add-or-update-team-project-permissions-legacy
    ///
    /// Add or update team project permissions (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team project permissions](https://docs.github.com/rest/reference/teams#add-or-update-team-project-permissions) endpoint.
    ///
    /// Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.
    PutTeamsteamIdProjectsprojectId(String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/projects/{project_id}`
    /// * docs https://docs.github.com/rest/reference/teams/#remove-a-project-from-a-team-legacy
    ///
    /// Remove a project from a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a project from a team](https://docs.github.com/rest/reference/teams#remove-a-project-from-a-team) endpoint.
    ///
    /// Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. **Note:** This endpoint removes the project from the team, but does not delete it.
    DeleteTeamsteamIdProjectsprojectId(String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/repos`
    /// * docs https://docs.github.com/rest/reference/teams/#list-team-repositories-legacy
    ///
    /// List team repositories (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/reference/teams#list-team-repositories) endpoint.
    GetTeamsteamIdRepos(String),
    /// * tags teams
    /// * get `/teams/{team_id}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository-legacy
    ///
    /// Check team permissions for a repository (Legacy)
    /// **Note**: Repositories inherited through a parent team will also be checked.
    ///
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-repository) endpoint.
    ///
    /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
    GetTeamsteamIdReposownerrepo(String, String, String),
    /// * tags teams
    /// * put `/teams/{team_id}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#add-or-update-team-repository-permissions-legacy
    ///
    /// Add or update team repository permissions (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Add or update team repository permissions](https://docs.github.com/rest/reference/teams#add-or-update-team-repository-permissions)" endpoint.
    ///
    /// To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization.
    ///
    /// Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutTeamsteamIdReposownerrepo(String, String, String),
    /// * tags teams
    /// * delete `/teams/{team_id}/repos/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/teams/#remove-a-repository-from-a-team-legacy
    ///
    /// Remove a repository from a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a repository from a team](https://docs.github.com/rest/reference/teams#remove-a-repository-from-a-team) endpoint.
    ///
    /// If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. NOTE: This does not delete the repository, it just removes it from the team.
    DeleteTeamsteamIdReposownerrepo(String, String, String),
    /// * tags teams
    /// * get `/teams/{team_id}/team-sync/group-mappings`
    /// * docs https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team-legacy
    ///
    /// List IdP groups for a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List IdP groups for a team`](https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups connected to a team on GitHub.
    GetTeamsteamIdTeamSyncGroupMappings(String),
    /// * tags teams
    /// * patch `/teams/{team_id}/team-sync/group-mappings`
    /// * docs https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections-legacy
    ///
    /// Create or update IdP group connections (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create or update IdP group connections`](https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
    PatchTeamsteamIdTeamSyncGroupMappings(String),
    /// * tags teams
    /// * get `/teams/{team_id}/teams`
    /// * docs https://docs.github.com/rest/reference/teams/#list-child-teams-legacy
    ///
    /// List child teams (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/reference/teams#list-child-teams) endpoint.
    GetTeamsteamIdTeams(String),
    /// * tags users
    /// * get `/user`
    /// * docs https://docs.github.com/rest/reference/users#get-the-authenticated-user
    ///
    /// Get the authenticated user
    /// If the authenticated user is authenticated through basic authentication or OAuth with the `user` scope, then the response lists public and private profile information.
    ///
    /// If the authenticated user is authenticated through OAuth without the `user` scope, then the response lists only public profile information.
    GetUser(),
    /// * tags users
    /// * patch `/user`
    /// * docs https://docs.github.com/rest/reference/users/#update-the-authenticated-user
    ///
    /// Update the authenticated user
    /// **Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.
    PatchUser(),
    /// * tags users
    /// * get `/user/blocks`
    /// * docs https://docs.github.com/rest/reference/users#list-users-blocked-by-the-authenticated-user
    ///
    /// List users blocked by the authenticated user
    /// List the users you've blocked on your personal account.
    GetUserBlocks(),
    /// * tags users
    /// * get `/user/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/users#check-if-a-user-is-blocked-by-the-authenticated-user
    ///
    /// Check if a user is blocked by the authenticated user
    ///
    GetUserBlocksusername(String),
    /// * tags users
    /// * put `/user/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/users#block-a-user
    ///
    /// Block a user
    ///
    PutUserBlocksusername(String),
    /// * tags users
    /// * delete `/user/blocks/{username}`
    /// * docs https://docs.github.com/rest/reference/users#unblock-a-user
    ///
    /// Unblock a user
    ///
    DeleteUserBlocksusername(String),
    /// * tags codespaces
    /// * get `/user/codespaces`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-codespaces-for-the-authenticated-user
    ///
    /// List codespaces for the authenticated user
    /// Lists the authenticated user's codespaces.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    GetUserCodespaces(),
    /// * tags codespaces
    /// * post `/user/codespaces`
    /// * docs https://docs.github.com/rest/reference/codespaces#create-a-codespace-for-the-authenticated-user
    ///
    /// Create a codespace for the authenticated user
    /// Creates a new codespace, owned by the authenticated user.
    ///
    /// This endpoint requires either a `repository_id` OR a `pull_request` but not both.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PostUserCodespaces(),
    /// * tags codespaces
    /// * get `/user/codespaces/secrets`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-secrets-for-the-authenticated-user
    ///
    /// List secrets for the authenticated user
    /// Lists all secrets available for a user's Codespaces without revealing their
    /// encrypted values.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    GetUserCodespacesSecrets(),
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/public-key`
    /// * docs https://docs.github.com/rest/reference/codespaces#get-public-key-for-the-authenticated-user
    ///
    /// Get public key for the authenticated user
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with one of the 'read:user' or 'user' scopes in their personal access token. User must have Codespaces access to use this endpoint.
    GetUserCodespacesSecretsPublicKey(),
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#get-a-secret-for-the-authenticated-user
    ///
    /// Get a secret for the authenticated user
    /// Gets a secret available to a user's codespaces without revealing its encrypted value.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    GetUserCodespacesSecretssecretName(String),
    /// * tags codespaces
    /// * put `/user/codespaces/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#create-or-update-a-secret-for-the-authenticated-user
    ///
    /// Create or update a secret for the authenticated user
    /// Creates or updates a secret for a user's codespace with an encrypted value. Encrypt your secret using
    /// [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access token with the `user` scope to use this endpoint. User must also have Codespaces access to use this endpoint.
    ///
    /// #### Example encrypting a secret using Node.js
    ///
    /// Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
    ///
    /// ```
    /// const sodium = require('tweetsodium');
    ///
    /// const key = "base64-encoded-public-key";
    /// const value = "plain-text-secret";
    ///
    /// // Convert the message and key to Uint8Array's (Buffer implements that interface)
    /// const messageBytes = Buffer.from(value);
    /// const keyBytes = Buffer.from(key, 'base64');
    ///
    /// // Encrypt using LibSodium.
    /// const encryptedBytes = sodium.seal(messageBytes, keyBytes);
    ///
    /// // Base64 the encrypted secret
    /// const encrypted = Buffer.from(encryptedBytes).toString('base64');
    ///
    /// console.log(encrypted);
    /// ```
    ///
    ///
    /// #### Example encrypting a secret using Python
    ///
    /// Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
    ///
    /// ```
    /// from base64 import b64encode
    /// from nacl import encoding, public
    ///
    /// def encrypt(public_key: str, secret_value: str) -> str:
    ///   """Encrypt a Unicode string using the public key."""
    ///   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
    ///   sealed_box = public.SealedBox(public_key)
    ///   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
    ///   return b64encode(encrypted).decode("utf-8")
    /// ```
    ///
    /// #### Example encrypting a secret using C#
    ///
    /// Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
    ///
    /// ```
    /// var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
    /// var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
    ///
    /// var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
    ///
    /// Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
    /// ```
    ///
    /// #### Example encrypting a secret using Ruby
    ///
    /// Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
    ///
    /// ```ruby
    /// require "rbnacl"
    /// require "base64"
    ///
    /// key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
    /// public_key = RbNaCl::PublicKey.new(key)
    ///
    /// box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
    /// encrypted_secret = box.encrypt("my_secret")
    ///
    /// # Print the base64 encoded secret
    /// puts Base64.strict_encode64(encrypted_secret)
    /// ```
    PutUserCodespacesSecretssecretName(String),
    /// * tags codespaces
    /// * delete `/user/codespaces/secrets/{secret_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#delete-a-secret-for-the-authenticated-user
    ///
    /// Delete a secret for the authenticated user
    /// Deletes a secret from a user's codespaces using the secret name. Deleting the secret will remove access from all codespaces that were allowed to access the secret. You must authenticate using an access token with the `user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    DeleteUserCodespacesSecretssecretName(String),
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/{secret_name}/repositories`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-selected-repositories-for-a-user-secret
    ///
    /// List selected repositories for a user secret
    /// List the repositories that have been granted the ability to use a user's codespace secret.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    GetUserCodespacesSecretssecretNameRepositories(String),
    /// * tags codespaces
    /// * put `/user/codespaces/secrets/{secret_name}/repositories`
    /// * docs https://docs.github.com/rest/reference/codespaces#set-selected-repositories-for-a-user-secret
    ///
    /// Set selected repositories for a user secret
    /// Select the repositories that will use a user's codespace secret.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    PutUserCodespacesSecretssecretNameRepositories(String),
    /// * tags codespaces
    /// * put `/user/codespaces/secrets/{secret_name}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/codespaces#add-a-selected-repository-to-a-user-secret
    ///
    /// Add a selected repository to a user secret
    /// Adds a repository to the selected repositories for a user's codespace secret.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    PutUserCodespacesSecretssecretNameRepositoriesrepositoryId(String, String),
    /// * tags codespaces
    /// * delete `/user/codespaces/secrets/{secret_name}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/codespaces#remove-a-selected-repository-from-a-user-secret
    ///
    /// Remove a selected repository from a user secret
    /// Removes a repository from the selected repositories for a user's codespace secret.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    DeleteUserCodespacesSecretssecretNameRepositoriesrepositoryId(String, String),
    /// * tags codespaces
    /// * get `/user/codespaces/{codespace_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#get-a-codespace-for-the-authenticated-user
    ///
    /// Get a codespace for the authenticated user
    /// Gets information about a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    GetUserCodespacescodespaceName(String),
    /// * tags codespaces
    /// * patch `/user/codespaces/{codespace_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#update-a-codespace-for-the-authenticated-user
    ///
    /// Update a codespace for the authenticated user
    /// Updates a codespace owned by the authenticated user. Currently only the codespace's machine type can be modified using this endpoint.
    ///
    /// Once you specify a new machine type it will be applied the next time your codespace is started.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PatchUserCodespacescodespaceName(String),
    /// * tags codespaces
    /// * delete `/user/codespaces/{codespace_name}`
    /// * docs https://docs.github.com/rest/reference/codespaces#delete-a-codespace-for-the-authenticated-user
    ///
    /// Delete a codespace for the authenticated user
    /// Deletes a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    DeleteUserCodespacescodespaceName(String),
    /// * tags codespaces
    /// * get `/user/codespaces/{codespace_name}/machines`
    /// * docs https://docs.github.com/rest/reference/codespaces#list-machine-types-for-a-codespace
    ///
    /// List machine types for a codespace
    /// List the machine types a codespace can transition to use.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    GetUserCodespacescodespaceNameMachines(String),
    /// * tags codespaces
    /// * post `/user/codespaces/{codespace_name}/start`
    /// * docs https://docs.github.com/rest/reference/codespaces#start-a-codespace-for-the-authenticated-user
    ///
    /// Start a codespace for the authenticated user
    /// Starts a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PostUserCodespacescodespaceNameStart(String),
    /// * tags codespaces
    /// * post `/user/codespaces/{codespace_name}/stop`
    /// * docs https://docs.github.com/rest/reference/codespaces#stop-a-codespace-for-the-authenticated-user
    ///
    /// Stop a codespace for the authenticated user
    /// Stops a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    PostUserCodespacescodespaceNameStop(String),
    /// * tags users
    /// * patch `/user/email/visibility`
    /// * docs https://docs.github.com/rest/reference/users#set-primary-email-visibility-for-the-authenticated-user
    ///
    /// Set primary email visibility for the authenticated user
    /// Sets the visibility for your primary email addresses.
    PatchUserEmailVisibility(),
    /// * tags users
    /// * get `/user/emails`
    /// * docs https://docs.github.com/rest/reference/users#list-email-addresses-for-the-authenticated-user
    ///
    /// List email addresses for the authenticated user
    /// Lists all of your email addresses, and specifies which one is visible to the public. This endpoint is accessible with the `user:email` scope.
    GetUserEmails(),
    /// * tags users
    /// * post `/user/emails`
    /// * docs https://docs.github.com/rest/reference/users#add-an-email-address-for-the-authenticated-user
    ///
    /// Add an email address for the authenticated user
    /// This endpoint is accessible with the `user` scope.
    PostUserEmails(),
    /// * tags users
    /// * delete `/user/emails`
    /// * docs https://docs.github.com/rest/reference/users#delete-an-email-address-for-the-authenticated-user
    ///
    /// Delete an email address for the authenticated user
    /// This endpoint is accessible with the `user` scope.
    DeleteUserEmails(),
    /// * tags users
    /// * get `/user/followers`
    /// * docs https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user
    ///
    /// List followers of the authenticated user
    /// Lists the people following the authenticated user.
    GetUserFollowers(),
    /// * tags users
    /// * get `/user/following`
    /// * docs https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows
    ///
    /// List the people the authenticated user follows
    /// Lists the people who the authenticated user follows.
    GetUserFollowing(),
    /// * tags users
    /// * get `/user/following/{username}`
    /// * docs https://docs.github.com/rest/reference/users#check-if-a-person-is-followed-by-the-authenticated-user
    ///
    /// Check if a person is followed by the authenticated user
    ///
    GetUserFollowingusername(String),
    /// * tags users
    /// * put `/user/following/{username}`
    /// * docs https://docs.github.com/rest/reference/users#follow-a-user
    ///
    /// Follow a user
    /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    ///
    /// Following a user requires the user to be logged in and authenticated with basic auth or OAuth with the `user:follow` scope.
    PutUserFollowingusername(String),
    /// * tags users
    /// * delete `/user/following/{username}`
    /// * docs https://docs.github.com/rest/reference/users#unfollow-a-user
    ///
    /// Unfollow a user
    /// Unfollowing a user requires the user to be logged in and authenticated with basic auth or OAuth with the `user:follow` scope.
    DeleteUserFollowingusername(String),
    /// * tags users
    /// * get `/user/gpg_keys`
    /// * docs https://docs.github.com/rest/reference/users#list-gpg-keys-for-the-authenticated-user
    ///
    /// List GPG keys for the authenticated user
    /// Lists the current user's GPG keys. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetUserGpgKeys(),
    /// * tags users
    /// * post `/user/gpg_keys`
    /// * docs https://docs.github.com/rest/reference/users#create-a-gpg-key-for-the-authenticated-user
    ///
    /// Create a GPG key for the authenticated user
    /// Adds a GPG key to the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth, or OAuth with at least `write:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    PostUserGpgKeys(),
    /// * tags users
    /// * get `/user/gpg_keys/{gpg_key_id}`
    /// * docs https://docs.github.com/rest/reference/users#get-a-gpg-key-for-the-authenticated-user
    ///
    /// Get a GPG key for the authenticated user
    /// View extended details for a single GPG key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetUserGpgKeysgpgKeyId(String),
    /// * tags users
    /// * delete `/user/gpg_keys/{gpg_key_id}`
    /// * docs https://docs.github.com/rest/reference/users#delete-a-gpg-key-for-the-authenticated-user
    ///
    /// Delete a GPG key for the authenticated user
    /// Removes a GPG key from the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `admin:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteUserGpgKeysgpgKeyId(String),
    /// * tags apps
    /// * get `/user/installations`
    /// * docs https://docs.github.com/rest/reference/apps#list-app-installations-accessible-to-the-user-access-token
    ///
    /// List app installations accessible to the user access token
    /// Lists installations of your GitHub App that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
    ///
    /// You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
    ///
    /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
    ///
    /// You can find the permissions for the installation under the `permissions` key.
    GetUserInstallations(),
    /// * tags apps
    /// * get `/user/installations/{installation_id}/repositories`
    /// * docs https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-user-access-token
    ///
    /// List repositories accessible to the user access token
    /// List repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access for an installation.
    ///
    /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
    ///
    /// You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
    ///
    /// The access the user has to each repository is included in the hash under the `permissions` key.
    GetUserInstallationsinstallationIdRepositories(String),
    /// * tags apps
    /// * put `/user/installations/{installation_id}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/apps#add-a-repository-to-an-app-installation
    ///
    /// Add a repository to an app installation
    /// Add a single repository to an installation. The authenticated user must have admin access to the repository.
    ///
    /// You must use a personal access token (which you can create via the [command line](https://docs.github.com/github/authenticating-to-github/creating-a-personal-access-token) or [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication)) to access this endpoint.
    PutUserInstallationsinstallationIdRepositoriesrepositoryId(String, String),
    /// * tags apps
    /// * delete `/user/installations/{installation_id}/repositories/{repository_id}`
    /// * docs https://docs.github.com/rest/reference/apps#remove-a-repository-from-an-app-installation
    ///
    /// Remove a repository from an app installation
    /// Remove a single repository from an installation. The authenticated user must have admin access to the repository.
    ///
    /// You must use a personal access token (which you can create via the [command line](https://docs.github.com/github/authenticating-to-github/creating-a-personal-access-token) or [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication)) to access this endpoint.
    DeleteUserInstallationsinstallationIdRepositoriesrepositoryId(String, String),
    /// * tags interactions
    /// * get `/user/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-your-public-repositories
    ///
    /// Get interaction restrictions for your public repositories
    /// Shows which type of GitHub user can interact with your public repositories and when the restriction expires.
    GetUserInteractionLimits(),
    /// * tags interactions
    /// * put `/user/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-your-public-repositories
    ///
    /// Set interaction restrictions for your public repositories
    /// Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.
    PutUserInteractionLimits(),
    /// * tags interactions
    /// * delete `/user/interaction-limits`
    /// * docs https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-from-your-public-repositories
    ///
    /// Remove interaction restrictions from your public repositories
    /// Removes any interaction restrictions from your public repositories.
    DeleteUserInteractionLimits(),
    /// * tags issues
    /// * get `/user/issues`
    /// * docs https://docs.github.com/rest/reference/issues#list-user-account-issues-assigned-to-the-authenticated-user
    ///
    /// List user account issues assigned to the authenticated user
    /// List issues across owned and member repositories assigned to the authenticated user.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetUserIssues(),
    /// * tags users
    /// * get `/user/keys`
    /// * docs https://docs.github.com/rest/reference/users#list-public-ssh-keys-for-the-authenticated-user
    ///
    /// List public SSH keys for the authenticated user
    /// Lists the public SSH keys for the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetUserKeys(),
    /// * tags users
    /// * post `/user/keys`
    /// * docs https://docs.github.com/rest/reference/users#create-a-public-ssh-key-for-the-authenticated-user
    ///
    /// Create a public SSH key for the authenticated user
    /// Adds a public SSH key to the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth, or OAuth with at least `write:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    PostUserKeys(),
    /// * tags users
    /// * get `/user/keys/{key_id}`
    /// * docs https://docs.github.com/rest/reference/users#get-a-public-ssh-key-for-the-authenticated-user
    ///
    /// Get a public SSH key for the authenticated user
    /// View extended details for a single public SSH key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    GetUserKeyskeyId(String),
    /// * tags users
    /// * delete `/user/keys/{key_id}`
    /// * docs https://docs.github.com/rest/reference/users#delete-a-public-ssh-key-for-the-authenticated-user
    ///
    /// Delete a public SSH key for the authenticated user
    /// Removes a public SSH key from the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `admin:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    DeleteUserKeyskeyId(String),
    /// * tags apps
    /// * get `/user/marketplace_purchases`
    /// * docs https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user
    ///
    /// List subscriptions for the authenticated user
    /// Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
    GetUserMarketplacePurchases(),
    /// * tags apps
    /// * get `/user/marketplace_purchases/stubbed`
    /// * docs https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user-stubbed
    ///
    /// List subscriptions for the authenticated user (stubbed)
    /// Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
    GetUserMarketplacePurchasesStubbed(),
    /// * tags orgs
    /// * get `/user/memberships/orgs`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organization-memberships-for-the-authenticated-user
    ///
    /// List organization memberships for the authenticated user
    ///
    GetUserMembershipsOrgs(),
    /// * tags orgs
    /// * get `/user/memberships/orgs/{org}`
    /// * docs https://docs.github.com/rest/reference/orgs#get-an-organization-membership-for-the-authenticated-user
    ///
    /// Get an organization membership for the authenticated user
    ///
    GetUserMembershipsOrgsorg(String),
    /// * tags orgs
    /// * patch `/user/memberships/orgs/{org}`
    /// * docs https://docs.github.com/rest/reference/orgs#update-an-organization-membership-for-the-authenticated-user
    ///
    /// Update an organization membership for the authenticated user
    ///
    PatchUserMembershipsOrgsorg(String),
    /// * tags migrations
    /// * get `/user/migrations`
    /// * docs https://docs.github.com/rest/reference/migrations#list-user-migrations
    ///
    /// List user migrations
    /// Lists all migrations a user has started.
    GetUserMigrations(),
    /// * tags migrations
    /// * post `/user/migrations`
    /// * docs https://docs.github.com/rest/reference/migrations#start-a-user-migration
    ///
    /// Start a user migration
    /// Initiates the generation of a user migration archive.
    PostUserMigrations(),
    /// * tags migrations
    /// * get `/user/migrations/{migration_id}`
    /// * docs https://docs.github.com/rest/reference/migrations#get-a-user-migration-status
    ///
    /// Get a user migration status
    /// Fetches a single user migration. The response includes the `state` of the migration, which can be one of the following values:
    ///
    /// *   `pending` - the migration hasn't started yet.
    /// *   `exporting` - the migration is in progress.
    /// *   `exported` - the migration finished successfully.
    /// *   `failed` - the migration failed.
    ///
    /// Once the migration has been `exported` you can [download the migration archive](https://docs.github.com/rest/reference/migrations#download-a-user-migration-archive).
    GetUserMigrationsmigrationId(String),
    /// * tags migrations
    /// * get `/user/migrations/{migration_id}/archive`
    /// * docs https://docs.github.com/rest/reference/migrations#download-a-user-migration-archive
    ///
    /// Download a user migration archive
    /// Fetches the URL to download the migration archive as a `tar.gz` file. Depending on the resources your repository uses, the migration archive can contain JSON files with data for these objects:
    ///
    /// *   attachments
    /// *   bases
    /// *   commit\_comments
    /// *   issue\_comments
    /// *   issue\_events
    /// *   issues
    /// *   milestones
    /// *   organizations
    /// *   projects
    /// *   protected\_branches
    /// *   pull\_request\_reviews
    /// *   pull\_requests
    /// *   releases
    /// *   repositories
    /// *   review\_comments
    /// *   schema
    /// *   users
    ///
    /// The archive will also contain an `attachments` directory that includes all attachment files uploaded to GitHub.com and a `repositories` directory that contains the repository's Git data.
    GetUserMigrationsmigrationIdArchive(String),
    /// * tags migrations
    /// * delete `/user/migrations/{migration_id}/archive`
    /// * docs https://docs.github.com/rest/reference/migrations#delete-a-user-migration-archive
    ///
    /// Delete a user migration archive
    /// Deletes a previous migration archive. Downloadable migration archives are automatically deleted after seven days. Migration metadata, which is returned in the [List user migrations](https://docs.github.com/rest/reference/migrations#list-user-migrations) and [Get a user migration status](https://docs.github.com/rest/reference/migrations#get-a-user-migration-status) endpoints, will continue to be available even after an archive is deleted.
    DeleteUserMigrationsmigrationIdArchive(String),
    /// * tags migrations
    /// * delete `/user/migrations/{migration_id}/repos/{repo_name}/lock`
    /// * docs https://docs.github.com/rest/reference/migrations#unlock-a-user-repository
    ///
    /// Unlock a user repository
    /// Unlocks a repository. You can lock repositories when you [start a user migration](https://docs.github.com/rest/reference/migrations#start-a-user-migration). Once the migration is complete you can unlock each repository to begin using it again or [delete the repository](https://docs.github.com/rest/reference/repos#delete-a-repository) if you no longer need the source data. Returns a status of `404 Not Found` if the repository is not locked.
    DeleteUserMigrationsmigrationIdReposrepoNameLock(String, String),
    /// * tags migrations
    /// * get `/user/migrations/{migration_id}/repositories`
    /// * docs https://docs.github.com/rest/reference/migrations#list-repositories-for-a-user-migration
    ///
    /// List repositories for a user migration
    /// Lists all the repositories for this user migration.
    GetUserMigrationsmigrationIdRepositories(String),
    /// * tags orgs
    /// * get `/user/orgs`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user
    ///
    /// List organizations for the authenticated user
    /// List organizations for the authenticated user.
    ///
    /// **OAuth scope requirements**
    ///
    /// This only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope. OAuth requests with insufficient scope receive a `403 Forbidden` response.
    GetUserOrgs(),
    /// * tags packages
    /// * get `/user/packages`
    /// * docs https://docs.github.com/rest/reference/packages#list-packages-for-the-authenticated-user
    ///
    /// List packages for the authenticated user's namespace
    /// Lists packages owned by the authenticated user within the user's namespace.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUserPackages(),
    /// * tags packages
    /// * get `/user/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-for-the-authenticated-user
    ///
    /// Get a package for the authenticated user
    /// Gets a specific package for a package owned by the authenticated user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUserPackagespackageTypepackageName(String, String),
    /// * tags packages
    /// * delete `/user/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-for-the-authenticated-user
    ///
    /// Delete a package for the authenticated user
    /// Deletes a package owned by the authenticated user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:delete` scopes.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    DeleteUserPackagespackageTypepackageName(String, String),
    /// * tags packages
    /// * post `/user/packages/{package_type}/{package_name}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-for-the-authenticated-user
    ///
    /// Restore a package for the authenticated user
    /// Restores a package owned by the authenticated user.
    ///
    /// You can restore a deleted package under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. If `package_type` is not `container`, your token must also include the `repo` scope.
    PostUserPackagespackageTypepackageNameRestore(String, String),
    /// * tags packages
    /// * get `/user/packages/{package_type}/{package_name}/versions`
    /// * docs https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-the-authenticated-user
    ///
    /// Get all package versions for a package owned by the authenticated user
    /// Returns all package versions for a package owned by the authenticated user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUserPackagespackageTypepackageNameVersions(String, String),
    /// * tags packages
    /// * get `/user/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-version-for-the-authenticated-user
    ///
    /// Get a package version for the authenticated user
    /// Gets a specific package version for a package owned by the authenticated user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUserPackagespackageTypepackageNameVersionspackageVersionId(String, String, String),
    /// * tags packages
    /// * delete `/user/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-version-for-the-authenticated-user
    ///
    /// Delete a package version for the authenticated user
    /// Deletes a specific package version for a package owned by the authenticated user.  If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    DeleteUserPackagespackageTypepackageNameVersionspackageVersionId(String, String, String),
    /// * tags packages
    /// * post `/user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-version-for-the-authenticated-user
    ///
    /// Restore a package version for the authenticated user
    /// Restores a package version owned by the authenticated user.
    ///
    /// You can restore a deleted package version under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. If `package_type` is not `container`, your token must also include the `repo` scope.
    PostUserPackagespackageTypepackageNameVersionspackageVersionIdRestore(String, String, String),
    /// * tags projects
    /// * post `/user/projects`
    /// * docs https://docs.github.com/rest/reference/projects#create-a-user-project
    ///
    /// Create a user project
    ///
    PostUserProjects(),
    /// * tags users
    /// * get `/user/public_emails`
    /// * docs https://docs.github.com/rest/reference/users#list-public-email-addresses-for-the-authenticated-user
    ///
    /// List public email addresses for the authenticated user
    /// Lists your publicly visible email address, which you can set with the [Set primary email visibility for the authenticated user](https://docs.github.com/rest/reference/users#set-primary-email-visibility-for-the-authenticated-user) endpoint. This endpoint is accessible with the `user:email` scope.
    GetUserPublicEmails(),
    /// * tags repos
    /// * get `/user/repos`
    /// * docs https://docs.github.com/rest/reference/repos#list-repositories-for-the-authenticated-user
    ///
    /// List repositories for the authenticated user
    /// Lists repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
    ///
    /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
    GetUserRepos(),
    /// * tags repos
    /// * post `/user/repos`
    /// * docs https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user
    ///
    /// Create a repository for the authenticated user
    /// Creates a new repository for the authenticated user.
    ///
    /// **OAuth scope requirements**
    ///
    /// When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
    ///
    /// *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
    /// *   `repo` scope to create a private repository.
    PostUserRepos(),
    /// * tags repos
    /// * get `/user/repository_invitations`
    /// * docs https://docs.github.com/rest/reference/repos#list-repository-invitations-for-the-authenticated-user
    ///
    /// List repository invitations for the authenticated user
    /// When authenticating as a user, this endpoint will list all currently open repository invitations for that user.
    GetUserRepositoryInvitations(),
    /// * tags repos
    /// * patch `/user/repository_invitations/{invitation_id}`
    /// * docs https://docs.github.com/rest/reference/repos#accept-a-repository-invitation
    ///
    /// Accept a repository invitation
    ///
    PatchUserRepositoryInvitationsinvitationId(String),
    /// * tags repos
    /// * delete `/user/repository_invitations/{invitation_id}`
    /// * docs https://docs.github.com/rest/reference/repos#decline-a-repository-invitation
    ///
    /// Decline a repository invitation
    ///
    DeleteUserRepositoryInvitationsinvitationId(String),
    /// * tags activity
    /// * get `/user/starred`
    /// * docs https://docs.github.com/rest/reference/activity#list-repositories-starred-by-the-authenticated-user
    ///
    /// List repositories starred by the authenticated user
    /// Lists repositories the authenticated user has starred.
    ///
    /// You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
    GetUserStarred(),
    /// * tags activity
    /// * get `/user/starred/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/activity#check-if-a-repository-is-starred-by-the-authenticated-user
    ///
    /// Check if a repository is starred by the authenticated user
    ///
    GetUserStarredownerrepo(String, String),
    /// * tags activity
    /// * put `/user/starred/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/activity#star-a-repository-for-the-authenticated-user
    ///
    /// Star a repository for the authenticated user
    /// Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
    PutUserStarredownerrepo(String, String),
    /// * tags activity
    /// * delete `/user/starred/{owner}/{repo}`
    /// * docs https://docs.github.com/rest/reference/activity#unstar-a-repository-for-the-authenticated-user
    ///
    /// Unstar a repository for the authenticated user
    ///
    DeleteUserStarredownerrepo(String, String),
    /// * tags activity
    /// * get `/user/subscriptions`
    /// * docs https://docs.github.com/rest/reference/activity#list-repositories-watched-by-the-authenticated-user
    ///
    /// List repositories watched by the authenticated user
    /// Lists repositories the authenticated user is watching.
    GetUserSubscriptions(),
    /// * tags teams
    /// * get `/user/teams`
    /// * docs https://docs.github.com/rest/reference/teams#list-teams-for-the-authenticated-user
    ///
    /// List teams for the authenticated user
    /// List all of the teams across all of the organizations to which the authenticated user belongs. This method requires `user`, `repo`, or `read:org` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/) when authenticating via [OAuth](https://docs.github.com/apps/building-oauth-apps/).
    GetUserTeams(),
    /// * tags users
    /// * get `/users`
    /// * docs https://docs.github.com/rest/reference/users#list-users
    ///
    /// List users
    /// Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.
    ///
    /// Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of users.
    GetUsers(),
    /// * tags users
    /// * get `/users/{username}`
    /// * docs https://docs.github.com/rest/reference/users#get-a-user
    ///
    /// Get a user
    /// Provides publicly available information about someone with a GitHub account.
    ///
    /// GitHub Apps with the `Plan` user permission can use this endpoint to retrieve information about a user's GitHub plan. The GitHub App must be authenticated as a user. See "[Identifying and authorizing users for GitHub Apps](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/)" for details about authentication. For an example response, see 'Response with GitHub plan information' below"
    ///
    /// The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/overview/resources-in-the-rest-api#authentication).
    ///
    /// The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see "[Emails API](https://docs.github.com/rest/reference/users#emails)".
    GetUsersusername(String),
    /// * tags activity
    /// * get `/users/{username}/events`
    /// * docs https://docs.github.com/rest/reference/activity#list-events-for-the-authenticated-user
    ///
    /// List events for the authenticated user
    /// If you are authenticated as the given user, you will see your private events. Otherwise, you'll only see public events.
    GetUsersusernameEvents(String),
    /// * tags activity
    /// * get `/users/{username}/events/orgs/{org}`
    /// * docs https://docs.github.com/rest/reference/activity#list-organization-events-for-the-authenticated-user
    ///
    /// List organization events for the authenticated user
    /// This is the user's organization dashboard. You must be authenticated as the user to view this.
    GetUsersusernameEventsOrgsorg(String, String),
    /// * tags activity
    /// * get `/users/{username}/events/public`
    /// * docs https://docs.github.com/rest/reference/activity#list-public-events-for-a-user
    ///
    /// List public events for a user
    ///
    GetUsersusernameEventsPublic(String),
    /// * tags users
    /// * get `/users/{username}/followers`
    /// * docs https://docs.github.com/rest/reference/users#list-followers-of-a-user
    ///
    /// List followers of a user
    /// Lists the people following the specified user.
    GetUsersusernameFollowers(String),
    /// * tags users
    /// * get `/users/{username}/following`
    /// * docs https://docs.github.com/rest/reference/users#list-the-people-a-user-follows
    ///
    /// List the people a user follows
    /// Lists the people who the specified user follows.
    GetUsersusernameFollowing(String),
    /// * tags users
    /// * get `/users/{username}/following/{target_user}`
    /// * docs https://docs.github.com/rest/reference/users#check-if-a-user-follows-another-user
    ///
    /// Check if a user follows another user
    ///
    GetUsersusernameFollowingtargetUser(String, String),
    /// * tags gists
    /// * get `/users/{username}/gists`
    /// * docs https://docs.github.com/rest/reference/gists#list-gists-for-a-user
    ///
    /// List gists for a user
    /// Lists public gists for the specified user:
    GetUsersusernameGists(String),
    /// * tags users
    /// * get `/users/{username}/gpg_keys`
    /// * docs https://docs.github.com/rest/reference/users#list-gpg-keys-for-a-user
    ///
    /// List GPG keys for a user
    /// Lists the GPG keys for a user. This information is accessible by anyone.
    GetUsersusernameGpgKeys(String),
    /// * tags users
    /// * get `/users/{username}/hovercard`
    /// * docs https://docs.github.com/rest/reference/users#get-contextual-information-for-a-user
    ///
    /// Get contextual information for a user
    /// Provides hovercard information when authenticated through basic auth or OAuth with the `repo` scope. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.
    ///
    /// The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository via cURL, it would look like this:
    ///
    /// ```shell
    ///  curl -u username:token
    ///   https://api.github.com/users/octocat/hovercard?subject_type=repository&subject_id=1300192
    /// ```
    GetUsersusernameHovercard(String),
    /// * tags apps
    /// * get `/users/{username}/installation`
    /// * docs https://docs.github.com/rest/reference/apps#get-a-user-installation-for-the-authenticated-app
    ///
    /// Get a user installation for the authenticated app
    /// Enables an authenticated GitHub App to find the user’s installation information.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    GetUsersusernameInstallation(String),
    /// * tags users
    /// * get `/users/{username}/keys`
    /// * docs https://docs.github.com/rest/reference/users#list-public-keys-for-a-user
    ///
    /// List public keys for a user
    /// Lists the _verified_ public SSH keys for a user. This is accessible by anyone.
    GetUsersusernameKeys(String),
    /// * tags orgs
    /// * get `/users/{username}/orgs`
    /// * docs https://docs.github.com/rest/reference/orgs#list-organizations-for-a-user
    ///
    /// List organizations for a user
    /// List [public organization memberships](https://help.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
    ///
    /// This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user) API instead.
    GetUsersusernameOrgs(String),
    /// * tags packages
    /// * get `/users/{username}/packages`
    /// * docs https://docs.github.com/rest/reference/packages#list-packages-for-user
    ///
    /// List packages for a user
    /// Lists all packages in a user's namespace for which the requesting user has access.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUsersusernamePackages(String),
    /// * tags packages
    /// * get `/users/{username}/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-for-a-user
    ///
    /// Get a package for a user
    /// Gets a specific package metadata for a public package owned by a user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUsersusernamePackagespackageTypepackageName(String, String, String),
    /// * tags packages
    /// * delete `/users/{username}/packages/{package_type}/{package_name}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-for-a-user
    ///
    /// Delete a package for a user
    /// Deletes an entire package for a user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
    DeleteUsersusernamePackagespackageTypepackageName(String, String, String),
    /// * tags packages
    /// * post `/users/{username}/packages/{package_type}/{package_name}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-for-a-user
    ///
    /// Restore a package for a user
    /// Restores an entire package for a user.
    ///
    /// You can restore a deleted package under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
    PostUsersusernamePackagespackageTypepackageNameRestore(String, String, String),
    /// * tags packages
    /// * get `/users/{username}/packages/{package_type}/{package_name}/versions`
    /// * docs https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-a-user
    ///
    /// Get all package versions for a package owned by a user
    /// Returns all package versions for a public package owned by a specified user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUsersusernamePackagespackageTypepackageNameVersions(String, String, String),
    /// * tags packages
    /// * get `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#get-a-package-version-for-a-user
    ///
    /// Get a package version for a user
    /// Gets a specific package version for a public package owned by a specified user.
    ///
    /// At this time, to use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(
        String,
        String,
        String,
        String,
    ),
    /// * tags packages
    /// * delete `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs https://docs.github.com/rest/reference/packages#delete-a-package-version-for-a-user
    ///
    /// Delete package version for a user
    /// Deletes a specific package version for a user. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
    DeleteUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(
        String,
        String,
        String,
        String,
    ),
    /// * tags packages
    /// * post `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore`
    /// * docs https://docs.github.com/rest/reference/packages#restore-a-package-version-for-a-user
    ///
    /// Restore package version for a user
    /// Restores a specific package version for a user.
    ///
    /// You can restore a deleted package under the following conditions:
    ///   - The package was deleted within the last 30 days.
    ///   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
    /// - If `package_type` is not `container`, your token must also include the `repo` scope.
    /// - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
    PostUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdRestore(
        String,
        String,
        String,
        String,
    ),
    /// * tags projects
    /// * get `/users/{username}/projects`
    /// * docs https://docs.github.com/rest/reference/projects#list-user-projects
    ///
    /// List user projects
    ///
    GetUsersusernameProjects(String),
    /// * tags activity
    /// * get `/users/{username}/received_events`
    /// * docs https://docs.github.com/rest/reference/activity#list-events-received-by-the-authenticated-user
    ///
    /// List events received by the authenticated user
    /// These are events that you've received by watching repos and following users. If you are authenticated as the given user, you will see private events. Otherwise, you'll only see public events.
    GetUsersusernameReceivedEvents(String),
    /// * tags activity
    /// * get `/users/{username}/received_events/public`
    /// * docs https://docs.github.com/rest/reference/activity#list-public-events-received-by-a-user
    ///
    /// List public events received by a user
    ///
    GetUsersusernameReceivedEventsPublic(String),
    /// * tags repos
    /// * get `/users/{username}/repos`
    /// * docs https://docs.github.com/rest/reference/repos#list-repositories-for-a-user
    ///
    /// List repositories for a user
    /// Lists public repositories for the specified user. Note: For GitHub AE, this endpoint will list internal repositories for the specified user.
    GetUsersusernameRepos(String),
    /// * tags billing
    /// * get `/users/{username}/settings/billing/actions`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-a-user
    ///
    /// Get GitHub Actions billing for a user
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Access tokens must have the `user` scope.
    GetUsersusernameSettingsBillingActions(String),
    /// * tags billing
    /// * get `/users/{username}/settings/billing/packages`
    /// * docs https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-a-user
    ///
    /// Get GitHub Packages billing for a user
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `user` scope.
    GetUsersusernameSettingsBillingPackages(String),
    /// * tags billing
    /// * get `/users/{username}/settings/billing/shared-storage`
    /// * docs https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-a-user
    ///
    /// Get shared storage billing for a user
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `user` scope.
    GetUsersusernameSettingsBillingSharedStorage(String),
    /// * tags activity
    /// * get `/users/{username}/starred`
    /// * docs https://docs.github.com/rest/reference/activity#list-repositories-starred-by-a-user
    ///
    /// List repositories starred by a user
    /// Lists repositories a user has starred.
    ///
    /// You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
    GetUsersusernameStarred(String),
    /// * tags activity
    /// * get `/users/{username}/subscriptions`
    /// * docs https://docs.github.com/rest/reference/activity#list-repositories-watched-by-a-user
    ///
    /// List repositories watched by a user
    /// Lists repositories a user is watching.
    GetUsersusernameSubscriptions(String),
    /// * tags meta
    /// * get `/zen`
    ///
    ///
    /// Get the Zen of GitHub
    /// Get a random sentence from the Zen of GitHub
    GetZen(),
}

impl EndPoints {
    pub const fn method(&self) -> Methods {
        match *self { EndPoints::Get(..) => Methods::Get,
  EndPoints::GetApp(..) => Methods::Get,
  EndPoints::PostAppManifestscodeConversions(..) => Methods::Post,
  EndPoints::GetAppHookConfig(..) => Methods::Get,
  EndPoints::PatchAppHookConfig(..) => Methods::Patch,
  EndPoints::GetAppHookDeliveries(..) => Methods::Get,
  EndPoints::GetAppHookDeliveriesdeliveryId(..) => Methods::Get,
  EndPoints::PostAppHookDeliveriesdeliveryIdAttempts(..) => Methods::Post,
  EndPoints::GetAppInstallations(..) => Methods::Get,
  EndPoints::GetAppInstallationsinstallationId(..) => Methods::Get,
  EndPoints::DeleteAppInstallationsinstallationId(..) => Methods::Delete,
  EndPoints::PostAppInstallationsinstallationIdAccessTokens(..) => Methods::Post,
  EndPoints::PutAppInstallationsinstallationIdSuspended(..) => Methods::Put,
  EndPoints::DeleteAppInstallationsinstallationIdSuspended(..) => Methods::Delete,
  EndPoints::GetApplicationsGrants(..) => Methods::Get,
  EndPoints::GetApplicationsGrantsgrantId(..) => Methods::Get,
  EndPoints::DeleteApplicationsGrantsgrantId(..) => Methods::Delete,
  EndPoints::DeleteApplicationsclientIdGrant(..) => Methods::Delete,
  EndPoints::PostApplicationsclientIdToken(..) => Methods::Post,
  EndPoints::PatchApplicationsclientIdToken(..) => Methods::Patch,
  EndPoints::DeleteApplicationsclientIdToken(..) => Methods::Delete,
  EndPoints::PostApplicationsclientIdTokenScoped(..) => Methods::Post,
  EndPoints::GetAppsappSlug(..) => Methods::Get,
  EndPoints::GetAuthorizations(..) => Methods::Get,
  EndPoints::PostAuthorizations(..) => Methods::Post,
  EndPoints::PutAuthorizationsClientsclientId(..) => Methods::Put,
  EndPoints::PutAuthorizationsClientsclientIdfingerprint(..) => Methods::Put,
  EndPoints::GetAuthorizationsauthorizationId(..) => Methods::Get,
  EndPoints::PatchAuthorizationsauthorizationId(..) => Methods::Patch,
  EndPoints::DeleteAuthorizationsauthorizationId(..) => Methods::Delete,
  EndPoints::GetCodesOfConduct(..) => Methods::Get,
  EndPoints::GetCodesOfConductkey(..) => Methods::Get,
  EndPoints::GetEmojis(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseActionsPermissions(..) => Methods::Get,
  EndPoints::PutEnterprisesenterpriseActionsPermissions(..) => Methods::Put,
  EndPoints::GetEnterprisesenterpriseActionsPermissionsOrganizations(..) => Methods::Get,
  EndPoints::PutEnterprisesenterpriseActionsPermissionsOrganizations(..) => Methods::Put,
  EndPoints::PutEnterprisesenterpriseActionsPermissionsOrganizationsorgId(..) => Methods::Put,
  EndPoints::DeleteEnterprisesenterpriseActionsPermissionsOrganizationsorgId(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseActionsPermissionsSelectedActions(..) => Methods::Get,
  EndPoints::PutEnterprisesenterpriseActionsPermissionsSelectedActions(..) => Methods::Put,
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroups(..) => Methods::Get,
  EndPoints::PostEnterprisesenterpriseActionsRunnerGroups(..) => Methods::Post,
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(..) => Methods::Get,
  EndPoints::PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(..) => Methods::Patch,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(..) => Methods::Get,
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(..) => Methods::Put,
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(..) => Methods::Put,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(..) => Methods::Get,
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(..) => Methods::Put,
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(..) => Methods::Put,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseActionsRunners(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseActionsRunnersDownloads(..) => Methods::Get,
  EndPoints::PostEnterprisesenterpriseActionsRunnersRegistrationToken(..) => Methods::Post,
  EndPoints::PostEnterprisesenterpriseActionsRunnersRemoveToken(..) => Methods::Post,
  EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerId(..) => Methods::Get,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerId(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(..) => Methods::Get,
  EndPoints::PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(..) => Methods::Post,
  EndPoints::PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(..) => Methods::Put,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(..) => Methods::Delete,
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(..) => Methods::Delete,
  EndPoints::GetEnterprisesenterpriseAuditLog(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseSettingsBillingActions(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseSettingsBillingPackages(..) => Methods::Get,
  EndPoints::GetEnterprisesenterpriseSettingsBillingSharedStorage(..) => Methods::Get,
  EndPoints::GetEvents(..) => Methods::Get,
  EndPoints::GetFeeds(..) => Methods::Get,
  EndPoints::GetGists(..) => Methods::Get,
  EndPoints::PostGists(..) => Methods::Post,
  EndPoints::GetGistsPublic(..) => Methods::Get,
  EndPoints::GetGistsStarred(..) => Methods::Get,
  EndPoints::GetGistsgistId(..) => Methods::Get,
  EndPoints::PatchGistsgistId(..) => Methods::Patch,
  EndPoints::DeleteGistsgistId(..) => Methods::Delete,
  EndPoints::GetGistsgistIdComments(..) => Methods::Get,
  EndPoints::PostGistsgistIdComments(..) => Methods::Post,
  EndPoints::GetGistsgistIdCommentscommentId(..) => Methods::Get,
  EndPoints::PatchGistsgistIdCommentscommentId(..) => Methods::Patch,
  EndPoints::DeleteGistsgistIdCommentscommentId(..) => Methods::Delete,
  EndPoints::GetGistsgistIdCommits(..) => Methods::Get,
  EndPoints::GetGistsgistIdForks(..) => Methods::Get,
  EndPoints::PostGistsgistIdForks(..) => Methods::Post,
  EndPoints::GetGistsgistIdStar(..) => Methods::Get,
  EndPoints::PutGistsgistIdStar(..) => Methods::Put,
  EndPoints::DeleteGistsgistIdStar(..) => Methods::Delete,
  EndPoints::GetGistsgistIdsha(..) => Methods::Get,
  EndPoints::GetGitignoreTemplates(..) => Methods::Get,
  EndPoints::GetGitignoreTemplatesname(..) => Methods::Get,
  EndPoints::GetInstallationRepositories(..) => Methods::Get,
  EndPoints::DeleteInstallationToken(..) => Methods::Delete,
  EndPoints::GetIssues(..) => Methods::Get,
  EndPoints::GetLicenses(..) => Methods::Get,
  EndPoints::GetLicenseslicense(..) => Methods::Get,
  EndPoints::PostMarkdown(..) => Methods::Post,
  EndPoints::PostMarkdownRaw(..) => Methods::Post,
  EndPoints::GetMarketplaceListingAccountsaccountId(..) => Methods::Get,
  EndPoints::GetMarketplaceListingPlans(..) => Methods::Get,
  EndPoints::GetMarketplaceListingPlansplanIdAccounts(..) => Methods::Get,
  EndPoints::GetMarketplaceListingStubbedAccountsaccountId(..) => Methods::Get,
  EndPoints::GetMarketplaceListingStubbedPlans(..) => Methods::Get,
  EndPoints::GetMarketplaceListingStubbedPlansplanIdAccounts(..) => Methods::Get,
  EndPoints::GetMeta(..) => Methods::Get,
  EndPoints::GetNetworksownerrepoEvents(..) => Methods::Get,
  EndPoints::GetNotifications(..) => Methods::Get,
  EndPoints::PutNotifications(..) => Methods::Put,
  EndPoints::GetNotificationsThreadsthreadId(..) => Methods::Get,
  EndPoints::PatchNotificationsThreadsthreadId(..) => Methods::Patch,
  EndPoints::GetNotificationsThreadsthreadIdSubscription(..) => Methods::Get,
  EndPoints::PutNotificationsThreadsthreadIdSubscription(..) => Methods::Put,
  EndPoints::DeleteNotificationsThreadsthreadIdSubscription(..) => Methods::Delete,
  EndPoints::GetOctocat(..) => Methods::Get,
  EndPoints::GetOrganizations(..) => Methods::Get,
  EndPoints::GetOrganizationsorganizationIdCustomRoles(..) => Methods::Get,
  EndPoints::GetOrgsorg(..) => Methods::Get,
  EndPoints::PatchOrgsorg(..) => Methods::Patch,
  EndPoints::GetOrgsorgActionsPermissions(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsPermissions(..) => Methods::Put,
  EndPoints::GetOrgsorgActionsPermissionsRepositories(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsPermissionsRepositories(..) => Methods::Put,
  EndPoints::PutOrgsorgActionsPermissionsRepositoriesrepositoryId(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsPermissionsRepositoriesrepositoryId(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsPermissionsSelectedActions(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsPermissionsSelectedActions(..) => Methods::Put,
  EndPoints::GetOrgsorgActionsRunnerGroups(..) => Methods::Get,
  EndPoints::PostOrgsorgActionsRunnerGroups(..) => Methods::Post,
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupId(..) => Methods::Get,
  EndPoints::PatchOrgsorgActionsRunnerGroupsrunnerGroupId(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupId(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(..) => Methods::Put,
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(..) => Methods::Put,
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsRunners(..) => Methods::Get,
  EndPoints::GetOrgsorgActionsRunnersDownloads(..) => Methods::Get,
  EndPoints::PostOrgsorgActionsRunnersRegistrationToken(..) => Methods::Post,
  EndPoints::PostOrgsorgActionsRunnersRemoveToken(..) => Methods::Post,
  EndPoints::GetOrgsorgActionsRunnersrunnerId(..) => Methods::Get,
  EndPoints::DeleteOrgsorgActionsRunnersrunnerId(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsRunnersrunnerIdLabels(..) => Methods::Get,
  EndPoints::PostOrgsorgActionsRunnersrunnerIdLabels(..) => Methods::Post,
  EndPoints::PutOrgsorgActionsRunnersrunnerIdLabels(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabels(..) => Methods::Delete,
  EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabelsname(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsSecrets(..) => Methods::Get,
  EndPoints::GetOrgsorgActionsSecretsPublicKey(..) => Methods::Get,
  EndPoints::GetOrgsorgActionsSecretssecretName(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsSecretssecretName(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsSecretssecretName(..) => Methods::Delete,
  EndPoints::GetOrgsorgActionsSecretssecretNameRepositories(..) => Methods::Get,
  EndPoints::PutOrgsorgActionsSecretssecretNameRepositories(..) => Methods::Put,
  EndPoints::PutOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(..) => Methods::Put,
  EndPoints::DeleteOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(..) => Methods::Delete,
  EndPoints::GetOrgsorgAuditLog(..) => Methods::Get,
  EndPoints::GetOrgsorgBlocks(..) => Methods::Get,
  EndPoints::GetOrgsorgBlocksusername(..) => Methods::Get,
  EndPoints::PutOrgsorgBlocksusername(..) => Methods::Put,
  EndPoints::DeleteOrgsorgBlocksusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgCredentialAuthorizations(..) => Methods::Get,
  EndPoints::DeleteOrgsorgCredentialAuthorizationscredentialId(..) => Methods::Delete,
  EndPoints::GetOrgsorgEvents(..) => Methods::Get,
  EndPoints::GetOrgsorgExternalGroupgroupId(..) => Methods::Get,
  EndPoints::GetOrgsorgExternalGroups(..) => Methods::Get,
  EndPoints::GetOrgsorgFailedInvitations(..) => Methods::Get,
  EndPoints::GetOrgsorgHooks(..) => Methods::Get,
  EndPoints::PostOrgsorgHooks(..) => Methods::Post,
  EndPoints::GetOrgsorgHookshookId(..) => Methods::Get,
  EndPoints::PatchOrgsorgHookshookId(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgHookshookId(..) => Methods::Delete,
  EndPoints::GetOrgsorgHookshookIdConfig(..) => Methods::Get,
  EndPoints::PatchOrgsorgHookshookIdConfig(..) => Methods::Patch,
  EndPoints::GetOrgsorgHookshookIdDeliveries(..) => Methods::Get,
  EndPoints::GetOrgsorgHookshookIdDeliveriesdeliveryId(..) => Methods::Get,
  EndPoints::PostOrgsorgHookshookIdDeliveriesdeliveryIdAttempts(..) => Methods::Post,
  EndPoints::PostOrgsorgHookshookIdPings(..) => Methods::Post,
  EndPoints::GetOrgsorgInstallation(..) => Methods::Get,
  EndPoints::GetOrgsorgInstallations(..) => Methods::Get,
  EndPoints::GetOrgsorgInteractionLimits(..) => Methods::Get,
  EndPoints::PutOrgsorgInteractionLimits(..) => Methods::Put,
  EndPoints::DeleteOrgsorgInteractionLimits(..) => Methods::Delete,
  EndPoints::GetOrgsorgInvitations(..) => Methods::Get,
  EndPoints::PostOrgsorgInvitations(..) => Methods::Post,
  EndPoints::DeleteOrgsorgInvitationsinvitationId(..) => Methods::Delete,
  EndPoints::GetOrgsorgInvitationsinvitationIdTeams(..) => Methods::Get,
  EndPoints::GetOrgsorgIssues(..) => Methods::Get,
  EndPoints::GetOrgsorgMembers(..) => Methods::Get,
  EndPoints::GetOrgsorgMembersusername(..) => Methods::Get,
  EndPoints::DeleteOrgsorgMembersusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgMembershipsusername(..) => Methods::Get,
  EndPoints::PutOrgsorgMembershipsusername(..) => Methods::Put,
  EndPoints::DeleteOrgsorgMembershipsusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgMigrations(..) => Methods::Get,
  EndPoints::PostOrgsorgMigrations(..) => Methods::Post,
  EndPoints::GetOrgsorgMigrationsmigrationId(..) => Methods::Get,
  EndPoints::GetOrgsorgMigrationsmigrationIdArchive(..) => Methods::Get,
  EndPoints::DeleteOrgsorgMigrationsmigrationIdArchive(..) => Methods::Delete,
  EndPoints::DeleteOrgsorgMigrationsmigrationIdReposrepoNameLock(..) => Methods::Delete,
  EndPoints::GetOrgsorgMigrationsmigrationIdRepositories(..) => Methods::Get,
  EndPoints::GetOrgsorgOutsideCollaborators(..) => Methods::Get,
  EndPoints::PutOrgsorgOutsideCollaboratorsusername(..) => Methods::Put,
  EndPoints::DeleteOrgsorgOutsideCollaboratorsusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgPackages(..) => Methods::Get,
  EndPoints::GetOrgsorgPackagespackageTypepackageName(..) => Methods::Get,
  EndPoints::DeleteOrgsorgPackagespackageTypepackageName(..) => Methods::Delete,
  EndPoints::PostOrgsorgPackagespackageTypepackageNameRestore(..) => Methods::Post,
  EndPoints::GetOrgsorgPackagespackageTypepackageNameVersions(..) => Methods::Get,
  EndPoints::GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Get,
  EndPoints::DeleteOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Delete,
  EndPoints::PostOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdRestore(..) => Methods::Post,
  EndPoints::GetOrgsorgProjects(..) => Methods::Get,
  EndPoints::PostOrgsorgProjects(..) => Methods::Post,
  EndPoints::GetOrgsorgPublicMembers(..) => Methods::Get,
  EndPoints::GetOrgsorgPublicMembersusername(..) => Methods::Get,
  EndPoints::PutOrgsorgPublicMembersusername(..) => Methods::Put,
  EndPoints::DeleteOrgsorgPublicMembersusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgRepos(..) => Methods::Get,
  EndPoints::PostOrgsorgRepos(..) => Methods::Post,
  EndPoints::GetOrgsorgSecretScanningAlerts(..) => Methods::Get,
  EndPoints::GetOrgsorgSettingsBillingActions(..) => Methods::Get,
  EndPoints::GetOrgsorgSettingsBillingAdvancedSecurity(..) => Methods::Get,
  EndPoints::GetOrgsorgSettingsBillingPackages(..) => Methods::Get,
  EndPoints::GetOrgsorgSettingsBillingSharedStorage(..) => Methods::Get,
  EndPoints::GetOrgsorgTeamSyncGroups(..) => Methods::Get,
  EndPoints::GetOrgsorgTeams(..) => Methods::Get,
  EndPoints::PostOrgsorgTeams(..) => Methods::Post,
  EndPoints::GetOrgsorgTeamsteamSlug(..) => Methods::Get,
  EndPoints::PatchOrgsorgTeamsteamSlug(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgTeamsteamSlug(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussions(..) => Methods::Get,
  EndPoints::PostOrgsorgTeamsteamSlugDiscussions(..) => Methods::Post,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(..) => Methods::Get,
  EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(..) => Methods::Get,
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(..) => Methods::Post,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Get,
  EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(..) => Methods::Get,
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(..) => Methods::Post,
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(..) => Methods::Get,
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(..) => Methods::Post,
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsreactionId(..) => Methods::Delete,
  EndPoints::PatchOrgsorgTeamsteamSlugExternalGroups(..) => Methods::Patch,
  EndPoints::DeleteOrgsorgTeamsteamSlugExternalGroups(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugInvitations(..) => Methods::Get,
  EndPoints::GetOrgsorgTeamsteamSlugMembers(..) => Methods::Get,
  EndPoints::GetOrgsorgTeamsteamSlugMembershipsusername(..) => Methods::Get,
  EndPoints::PutOrgsorgTeamsteamSlugMembershipsusername(..) => Methods::Put,
  EndPoints::DeleteOrgsorgTeamsteamSlugMembershipsusername(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugProjects(..) => Methods::Get,
  EndPoints::GetOrgsorgTeamsteamSlugProjectsprojectId(..) => Methods::Get,
  EndPoints::PutOrgsorgTeamsteamSlugProjectsprojectId(..) => Methods::Put,
  EndPoints::DeleteOrgsorgTeamsteamSlugProjectsprojectId(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugRepos(..) => Methods::Get,
  EndPoints::GetOrgsorgTeamsteamSlugReposownerrepo(..) => Methods::Get,
  EndPoints::PutOrgsorgTeamsteamSlugReposownerrepo(..) => Methods::Put,
  EndPoints::DeleteOrgsorgTeamsteamSlugReposownerrepo(..) => Methods::Delete,
  EndPoints::GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(..) => Methods::Get,
  EndPoints::PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(..) => Methods::Patch,
  EndPoints::GetOrgsorgTeamsteamSlugTeams(..) => Methods::Get,
  EndPoints::GetProjectsColumnsCardscardId(..) => Methods::Get,
  EndPoints::PatchProjectsColumnsCardscardId(..) => Methods::Patch,
  EndPoints::DeleteProjectsColumnsCardscardId(..) => Methods::Delete,
  EndPoints::PostProjectsColumnsCardscardIdMoves(..) => Methods::Post,
  EndPoints::GetProjectsColumnscolumnId(..) => Methods::Get,
  EndPoints::PatchProjectsColumnscolumnId(..) => Methods::Patch,
  EndPoints::DeleteProjectsColumnscolumnId(..) => Methods::Delete,
  EndPoints::GetProjectsColumnscolumnIdCards(..) => Methods::Get,
  EndPoints::PostProjectsColumnscolumnIdCards(..) => Methods::Post,
  EndPoints::PostProjectsColumnscolumnIdMoves(..) => Methods::Post,
  EndPoints::GetProjectsprojectId(..) => Methods::Get,
  EndPoints::PatchProjectsprojectId(..) => Methods::Patch,
  EndPoints::DeleteProjectsprojectId(..) => Methods::Delete,
  EndPoints::GetProjectsprojectIdCollaborators(..) => Methods::Get,
  EndPoints::PutProjectsprojectIdCollaboratorsusername(..) => Methods::Put,
  EndPoints::DeleteProjectsprojectIdCollaboratorsusername(..) => Methods::Delete,
  EndPoints::GetProjectsprojectIdCollaboratorsusernamePermission(..) => Methods::Get,
  EndPoints::GetProjectsprojectIdColumns(..) => Methods::Get,
  EndPoints::PostProjectsprojectIdColumns(..) => Methods::Post,
  EndPoints::GetRateLimit(..) => Methods::Get,
  EndPoints::DeleteReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetReposownerrepo(..) => Methods::Get,
  EndPoints::PatchReposownerrepo(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepo(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsArtifacts(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsArtifactsartifactId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoActionsArtifactsartifactId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsArtifactsartifactIdarchiveFormat(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsJobsjobId(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsJobsjobIdLogs(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsPermissions(..) => Methods::Get,
  EndPoints::PutReposownerrepoActionsPermissions(..) => Methods::Put,
  EndPoints::GetReposownerrepoActionsPermissionsSelectedActions(..) => Methods::Get,
  EndPoints::PutReposownerrepoActionsPermissionsSelectedActions(..) => Methods::Put,
  EndPoints::GetReposownerrepoActionsRunners(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunnersDownloads(..) => Methods::Get,
  EndPoints::PostReposownerrepoActionsRunnersRegistrationToken(..) => Methods::Post,
  EndPoints::PostReposownerrepoActionsRunnersRemoveToken(..) => Methods::Post,
  EndPoints::GetReposownerrepoActionsRunnersrunnerId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsRunnersrunnerIdLabels(..) => Methods::Get,
  EndPoints::PostReposownerrepoActionsRunnersrunnerIdLabels(..) => Methods::Post,
  EndPoints::PutReposownerrepoActionsRunnersrunnerIdLabels(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabels(..) => Methods::Delete,
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsRuns(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunsrunId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoActionsRunsrunId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsRunsrunIdApprovals(..) => Methods::Get,
  EndPoints::PostReposownerrepoActionsRunsrunIdApprove(..) => Methods::Post,
  EndPoints::GetReposownerrepoActionsRunsrunIdArtifacts(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberLogs(..) => Methods::Get,
  EndPoints::PostReposownerrepoActionsRunsrunIdCancel(..) => Methods::Post,
  EndPoints::GetReposownerrepoActionsRunsrunIdJobs(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsRunsrunIdLogs(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoActionsRunsrunIdLogs(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsRunsrunIdPendingDeployments(..) => Methods::Get,
  EndPoints::PostReposownerrepoActionsRunsrunIdPendingDeployments(..) => Methods::Post,
  EndPoints::PostReposownerrepoActionsRunsrunIdRerun(..) => Methods::Post,
  EndPoints::GetReposownerrepoActionsRunsrunIdTiming(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsSecrets(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsSecretsPublicKey(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsSecretssecretName(..) => Methods::Get,
  EndPoints::PutReposownerrepoActionsSecretssecretName(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoActionsSecretssecretName(..) => Methods::Delete,
  EndPoints::GetReposownerrepoActionsWorkflows(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowId(..) => Methods::Get,
  EndPoints::PutReposownerrepoActionsWorkflowsworkflowIdDisable(..) => Methods::Put,
  EndPoints::PostReposownerrepoActionsWorkflowsworkflowIdDispatches(..) => Methods::Post,
  EndPoints::PutReposownerrepoActionsWorkflowsworkflowIdEnable(..) => Methods::Put,
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdRuns(..) => Methods::Get,
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdTiming(..) => Methods::Get,
  EndPoints::GetReposownerrepoAssignees(..) => Methods::Get,
  EndPoints::GetReposownerrepoAssigneesassignee(..) => Methods::Get,
  EndPoints::GetReposownerrepoAutolinks(..) => Methods::Get,
  EndPoints::PostReposownerrepoAutolinks(..) => Methods::Post,
  EndPoints::GetReposownerrepoAutolinksautolinkId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoAutolinksautolinkId(..) => Methods::Delete,
  EndPoints::PutReposownerrepoAutomatedSecurityFixes(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoAutomatedSecurityFixes(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranches(..) => Methods::Get,
  EndPoints::GetReposownerrepoBranchesbranch(..) => Methods::Get,
  EndPoints::GetReposownerrepoBranchesbranchProtection(..) => Methods::Get,
  EndPoints::PutReposownerrepoBranchesbranchProtection(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoBranchesbranchProtection(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionEnforceAdmins(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionEnforceAdmins(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionEnforceAdmins(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(..) => Methods::Get,
  EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredSignatures(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredSignatures(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredSignatures(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(..) => Methods::Get,
  EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecks(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(..) => Methods::Post,
  EndPoints::PutReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictions(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictions(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsApps(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsApps(..) => Methods::Post,
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsApps(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsApps(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsTeams(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsTeams(..) => Methods::Post,
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsTeams(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsTeams(..) => Methods::Delete,
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsUsers(..) => Methods::Get,
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsUsers(..) => Methods::Post,
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsUsers(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsUsers(..) => Methods::Delete,
  EndPoints::PostReposownerrepoBranchesbranchRename(..) => Methods::Post,
  EndPoints::PostReposownerrepoCheckRuns(..) => Methods::Post,
  EndPoints::GetReposownerrepoCheckRunscheckRunId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoCheckRunscheckRunId(..) => Methods::Patch,
  EndPoints::GetReposownerrepoCheckRunscheckRunIdAnnotations(..) => Methods::Get,
  EndPoints::PostReposownerrepoCheckRunscheckRunIdRerequest(..) => Methods::Post,
  EndPoints::PostReposownerrepoCheckSuites(..) => Methods::Post,
  EndPoints::PatchReposownerrepoCheckSuitesPreferences(..) => Methods::Patch,
  EndPoints::GetReposownerrepoCheckSuitescheckSuiteId(..) => Methods::Get,
  EndPoints::GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(..) => Methods::Get,
  EndPoints::PostReposownerrepoCheckSuitescheckSuiteIdRerequest(..) => Methods::Post,
  EndPoints::GetReposownerrepoCodeScanningAlerts(..) => Methods::Get,
  EndPoints::GetReposownerrepoCodeScanningAlertsalertNumber(..) => Methods::Get,
  EndPoints::PatchReposownerrepoCodeScanningAlertsalertNumber(..) => Methods::Patch,
  EndPoints::GetReposownerrepoCodeScanningAlertsalertNumberInstances(..) => Methods::Get,
  EndPoints::GetReposownerrepoCodeScanningAnalyses(..) => Methods::Get,
  EndPoints::GetReposownerrepoCodeScanningAnalysesanalysisId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoCodeScanningAnalysesanalysisId(..) => Methods::Delete,
  EndPoints::PostReposownerrepoCodeScanningSarifs(..) => Methods::Post,
  EndPoints::GetReposownerrepoCodeScanningSarifssarifId(..) => Methods::Get,
  EndPoints::GetReposownerrepoCodespaces(..) => Methods::Get,
  EndPoints::PostReposownerrepoCodespaces(..) => Methods::Post,
  EndPoints::GetReposownerrepoCodespacesMachines(..) => Methods::Get,
  EndPoints::GetReposownerrepoCollaborators(..) => Methods::Get,
  EndPoints::GetReposownerrepoCollaboratorsusername(..) => Methods::Get,
  EndPoints::PutReposownerrepoCollaboratorsusername(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoCollaboratorsusername(..) => Methods::Delete,
  EndPoints::GetReposownerrepoCollaboratorsusernamePermission(..) => Methods::Get,
  EndPoints::GetReposownerrepoComments(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommentscommentId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoCommentscommentId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoCommentscommentId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoCommentscommentIdReactions(..) => Methods::Get,
  EndPoints::PostReposownerrepoCommentscommentIdReactions(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoCommentscommentIdReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoCommits(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitscommitShaBranchesWhereHead(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitscommitShaComments(..) => Methods::Get,
  EndPoints::PostReposownerrepoCommitscommitShaComments(..) => Methods::Post,
  EndPoints::GetReposownerrepoCommitscommitShaPulls(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitsref(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitsrefCheckRuns(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitsrefCheckSuites(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitsrefStatus(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommitsrefStatuses(..) => Methods::Get,
  EndPoints::GetReposownerrepoCommunityProfile(..) => Methods::Get,
  EndPoints::GetReposownerrepoComparebasehead(..) => Methods::Get,
  EndPoints::GetReposownerrepoContentspath(..) => Methods::Get,
  EndPoints::PutReposownerrepoContentspath(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoContentspath(..) => Methods::Delete,
  EndPoints::GetReposownerrepoContributors(..) => Methods::Get,
  EndPoints::GetReposownerrepoDeployments(..) => Methods::Get,
  EndPoints::PostReposownerrepoDeployments(..) => Methods::Post,
  EndPoints::GetReposownerrepoDeploymentsdeploymentId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoDeploymentsdeploymentId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatuses(..) => Methods::Get,
  EndPoints::PostReposownerrepoDeploymentsdeploymentIdStatuses(..) => Methods::Post,
  EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(..) => Methods::Get,
  EndPoints::PostReposownerrepoDispatches(..) => Methods::Post,
  EndPoints::GetReposownerrepoEnvironments(..) => Methods::Get,
  EndPoints::GetReposownerrepoEnvironmentsenvironmentName(..) => Methods::Get,
  EndPoints::PutReposownerrepoEnvironmentsenvironmentName(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoEnvironmentsenvironmentName(..) => Methods::Delete,
  EndPoints::GetReposownerrepoEvents(..) => Methods::Get,
  EndPoints::GetReposownerrepoForks(..) => Methods::Get,
  EndPoints::PostReposownerrepoForks(..) => Methods::Post,
  EndPoints::PostReposownerrepoGitBlobs(..) => Methods::Post,
  EndPoints::GetReposownerrepoGitBlobsfileSha(..) => Methods::Get,
  EndPoints::PostReposownerrepoGitCommits(..) => Methods::Post,
  EndPoints::GetReposownerrepoGitCommitscommitSha(..) => Methods::Get,
  EndPoints::GetReposownerrepoGitMatchingRefsref(..) => Methods::Get,
  EndPoints::GetReposownerrepoGitRefref(..) => Methods::Get,
  EndPoints::PostReposownerrepoGitRefs(..) => Methods::Post,
  EndPoints::PatchReposownerrepoGitRefsref(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoGitRefsref(..) => Methods::Delete,
  EndPoints::PostReposownerrepoGitTags(..) => Methods::Post,
  EndPoints::GetReposownerrepoGitTagstagSha(..) => Methods::Get,
  EndPoints::PostReposownerrepoGitTrees(..) => Methods::Post,
  EndPoints::GetReposownerrepoGitTreestreeSha(..) => Methods::Get,
  EndPoints::GetReposownerrepoHooks(..) => Methods::Get,
  EndPoints::PostReposownerrepoHooks(..) => Methods::Post,
  EndPoints::GetReposownerrepoHookshookId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoHookshookId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoHookshookId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoHookshookIdConfig(..) => Methods::Get,
  EndPoints::PatchReposownerrepoHookshookIdConfig(..) => Methods::Patch,
  EndPoints::GetReposownerrepoHookshookIdDeliveries(..) => Methods::Get,
  EndPoints::GetReposownerrepoHookshookIdDeliveriesdeliveryId(..) => Methods::Get,
  EndPoints::PostReposownerrepoHookshookIdDeliveriesdeliveryIdAttempts(..) => Methods::Post,
  EndPoints::PostReposownerrepoHookshookIdPings(..) => Methods::Post,
  EndPoints::PostReposownerrepoHookshookIdTests(..) => Methods::Post,
  EndPoints::GetReposownerrepoImport(..) => Methods::Get,
  EndPoints::PutReposownerrepoImport(..) => Methods::Put,
  EndPoints::PatchReposownerrepoImport(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoImport(..) => Methods::Delete,
  EndPoints::GetReposownerrepoImportAuthors(..) => Methods::Get,
  EndPoints::PatchReposownerrepoImportAuthorsauthorId(..) => Methods::Patch,
  EndPoints::GetReposownerrepoImportLargeFiles(..) => Methods::Get,
  EndPoints::PatchReposownerrepoImportLfs(..) => Methods::Patch,
  EndPoints::GetReposownerrepoInstallation(..) => Methods::Get,
  EndPoints::GetReposownerrepoInteractionLimits(..) => Methods::Get,
  EndPoints::PutReposownerrepoInteractionLimits(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoInteractionLimits(..) => Methods::Delete,
  EndPoints::GetReposownerrepoInvitations(..) => Methods::Get,
  EndPoints::PatchReposownerrepoInvitationsinvitationId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoInvitationsinvitationId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssues(..) => Methods::Get,
  EndPoints::PostReposownerrepoIssues(..) => Methods::Post,
  EndPoints::GetReposownerrepoIssuesComments(..) => Methods::Get,
  EndPoints::GetReposownerrepoIssuesCommentscommentId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoIssuesCommentscommentId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoIssuesCommentscommentId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssuesCommentscommentIdReactions(..) => Methods::Get,
  EndPoints::PostReposownerrepoIssuesCommentscommentIdReactions(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoIssuesCommentscommentIdReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssuesEvents(..) => Methods::Get,
  EndPoints::GetReposownerrepoIssuesEventseventId(..) => Methods::Get,
  EndPoints::GetReposownerrepoIssuesissueNumber(..) => Methods::Get,
  EndPoints::PatchReposownerrepoIssuesissueNumber(..) => Methods::Patch,
  EndPoints::PostReposownerrepoIssuesissueNumberAssignees(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoIssuesissueNumberAssignees(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssuesissueNumberComments(..) => Methods::Get,
  EndPoints::PostReposownerrepoIssuesissueNumberComments(..) => Methods::Post,
  EndPoints::GetReposownerrepoIssuesissueNumberEvents(..) => Methods::Get,
  EndPoints::GetReposownerrepoIssuesissueNumberLabels(..) => Methods::Get,
  EndPoints::PostReposownerrepoIssuesissueNumberLabels(..) => Methods::Post,
  EndPoints::PutReposownerrepoIssuesissueNumberLabels(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoIssuesissueNumberLabels(..) => Methods::Delete,
  EndPoints::DeleteReposownerrepoIssuesissueNumberLabelsname(..) => Methods::Delete,
  EndPoints::PutReposownerrepoIssuesissueNumberLock(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoIssuesissueNumberLock(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssuesissueNumberReactions(..) => Methods::Get,
  EndPoints::PostReposownerrepoIssuesissueNumberReactions(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoIssuesissueNumberReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoIssuesissueNumberTimeline(..) => Methods::Get,
  EndPoints::GetReposownerrepoKeys(..) => Methods::Get,
  EndPoints::PostReposownerrepoKeys(..) => Methods::Post,
  EndPoints::GetReposownerrepoKeyskeyId(..) => Methods::Get,
  EndPoints::DeleteReposownerrepoKeyskeyId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoLabels(..) => Methods::Get,
  EndPoints::PostReposownerrepoLabels(..) => Methods::Post,
  EndPoints::GetReposownerrepoLabelsname(..) => Methods::Get,
  EndPoints::PatchReposownerrepoLabelsname(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoLabelsname(..) => Methods::Delete,
  EndPoints::GetReposownerrepoLanguages(..) => Methods::Get,
  EndPoints::PutReposownerrepoLfs(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoLfs(..) => Methods::Delete,
  EndPoints::GetReposownerrepoLicense(..) => Methods::Get,
  EndPoints::PostReposownerrepoMergeUpstream(..) => Methods::Post,
  EndPoints::PostReposownerrepoMerges(..) => Methods::Post,
  EndPoints::GetReposownerrepoMilestones(..) => Methods::Get,
  EndPoints::PostReposownerrepoMilestones(..) => Methods::Post,
  EndPoints::GetReposownerrepoMilestonesmilestoneNumber(..) => Methods::Get,
  EndPoints::PatchReposownerrepoMilestonesmilestoneNumber(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoMilestonesmilestoneNumber(..) => Methods::Delete,
  EndPoints::GetReposownerrepoMilestonesmilestoneNumberLabels(..) => Methods::Get,
  EndPoints::GetReposownerrepoNotifications(..) => Methods::Get,
  EndPoints::PutReposownerrepoNotifications(..) => Methods::Put,
  EndPoints::GetReposownerrepoPages(..) => Methods::Get,
  EndPoints::PostReposownerrepoPages(..) => Methods::Post,
  EndPoints::PutReposownerrepoPages(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoPages(..) => Methods::Delete,
  EndPoints::GetReposownerrepoPagesBuilds(..) => Methods::Get,
  EndPoints::PostReposownerrepoPagesBuilds(..) => Methods::Post,
  EndPoints::GetReposownerrepoPagesBuildsLatest(..) => Methods::Get,
  EndPoints::GetReposownerrepoPagesBuildsbuildId(..) => Methods::Get,
  EndPoints::GetReposownerrepoPagesHealth(..) => Methods::Get,
  EndPoints::GetReposownerrepoProjects(..) => Methods::Get,
  EndPoints::PostReposownerrepoProjects(..) => Methods::Post,
  EndPoints::GetReposownerrepoPulls(..) => Methods::Get,
  EndPoints::PostReposownerrepoPulls(..) => Methods::Post,
  EndPoints::GetReposownerrepoPullsComments(..) => Methods::Get,
  EndPoints::GetReposownerrepoPullsCommentscommentId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoPullsCommentscommentId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoPullsCommentscommentId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoPullsCommentscommentIdReactions(..) => Methods::Get,
  EndPoints::PostReposownerrepoPullsCommentscommentIdReactions(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoPullsCommentscommentIdReactionsreactionId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoPullspullNumber(..) => Methods::Get,
  EndPoints::PatchReposownerrepoPullspullNumber(..) => Methods::Patch,
  EndPoints::PostReposownerrepoPullspullNumberCodespaces(..) => Methods::Post,
  EndPoints::GetReposownerrepoPullspullNumberComments(..) => Methods::Get,
  EndPoints::PostReposownerrepoPullspullNumberComments(..) => Methods::Post,
  EndPoints::PostReposownerrepoPullspullNumberCommentscommentIdReplies(..) => Methods::Post,
  EndPoints::GetReposownerrepoPullspullNumberCommits(..) => Methods::Get,
  EndPoints::GetReposownerrepoPullspullNumberFiles(..) => Methods::Get,
  EndPoints::GetReposownerrepoPullspullNumberMerge(..) => Methods::Get,
  EndPoints::PutReposownerrepoPullspullNumberMerge(..) => Methods::Put,
  EndPoints::GetReposownerrepoPullspullNumberRequestedReviewers(..) => Methods::Get,
  EndPoints::PostReposownerrepoPullspullNumberRequestedReviewers(..) => Methods::Post,
  EndPoints::DeleteReposownerrepoPullspullNumberRequestedReviewers(..) => Methods::Delete,
  EndPoints::GetReposownerrepoPullspullNumberReviews(..) => Methods::Get,
  EndPoints::PostReposownerrepoPullspullNumberReviews(..) => Methods::Post,
  EndPoints::GetReposownerrepoPullspullNumberReviewsreviewId(..) => Methods::Get,
  EndPoints::PutReposownerrepoPullspullNumberReviewsreviewId(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoPullspullNumberReviewsreviewId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoPullspullNumberReviewsreviewIdComments(..) => Methods::Get,
  EndPoints::PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(..) => Methods::Put,
  EndPoints::PostReposownerrepoPullspullNumberReviewsreviewIdEvents(..) => Methods::Post,
  EndPoints::PutReposownerrepoPullspullNumberUpdateBranch(..) => Methods::Put,
  EndPoints::GetReposownerrepoReadme(..) => Methods::Get,
  EndPoints::GetReposownerrepoReadmedir(..) => Methods::Get,
  EndPoints::GetReposownerrepoReleases(..) => Methods::Get,
  EndPoints::PostReposownerrepoReleases(..) => Methods::Post,
  EndPoints::GetReposownerrepoReleasesAssetsassetId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoReleasesAssetsassetId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoReleasesAssetsassetId(..) => Methods::Delete,
  EndPoints::PostReposownerrepoReleasesGenerateNotes(..) => Methods::Post,
  EndPoints::GetReposownerrepoReleasesLatest(..) => Methods::Get,
  EndPoints::GetReposownerrepoReleasesTagstag(..) => Methods::Get,
  EndPoints::GetReposownerrepoReleasesreleaseId(..) => Methods::Get,
  EndPoints::PatchReposownerrepoReleasesreleaseId(..) => Methods::Patch,
  EndPoints::DeleteReposownerrepoReleasesreleaseId(..) => Methods::Delete,
  EndPoints::GetReposownerrepoReleasesreleaseIdAssets(..) => Methods::Get,
  EndPoints::PostReposownerrepoReleasesreleaseIdAssets(..) => Methods::Post,
  EndPoints::PostReposownerrepoReleasesreleaseIdReactions(..) => Methods::Post,
  EndPoints::GetReposownerrepoSecretScanningAlerts(..) => Methods::Get,
  EndPoints::GetReposownerrepoSecretScanningAlertsalertNumber(..) => Methods::Get,
  EndPoints::PatchReposownerrepoSecretScanningAlertsalertNumber(..) => Methods::Patch,
  EndPoints::GetReposownerrepoSecretScanningAlertsalertNumberLocations(..) => Methods::Get,
  EndPoints::GetReposownerrepoStargazers(..) => Methods::Get,
  EndPoints::GetReposownerrepoStatsCodeFrequency(..) => Methods::Get,
  EndPoints::GetReposownerrepoStatsCommitActivity(..) => Methods::Get,
  EndPoints::GetReposownerrepoStatsContributors(..) => Methods::Get,
  EndPoints::GetReposownerrepoStatsParticipation(..) => Methods::Get,
  EndPoints::GetReposownerrepoStatsPunchCard(..) => Methods::Get,
  EndPoints::PostReposownerrepoStatusessha(..) => Methods::Post,
  EndPoints::GetReposownerrepoSubscribers(..) => Methods::Get,
  EndPoints::GetReposownerrepoSubscription(..) => Methods::Get,
  EndPoints::PutReposownerrepoSubscription(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoSubscription(..) => Methods::Delete,
  EndPoints::GetReposownerrepoTags(..) => Methods::Get,
  EndPoints::GetReposownerrepoTarballref(..) => Methods::Get,
  EndPoints::GetReposownerrepoTeams(..) => Methods::Get,
  EndPoints::GetReposownerrepoTopics(..) => Methods::Get,
  EndPoints::PutReposownerrepoTopics(..) => Methods::Put,
  EndPoints::GetReposownerrepoTrafficClones(..) => Methods::Get,
  EndPoints::GetReposownerrepoTrafficPopularPaths(..) => Methods::Get,
  EndPoints::GetReposownerrepoTrafficPopularReferrers(..) => Methods::Get,
  EndPoints::GetReposownerrepoTrafficViews(..) => Methods::Get,
  EndPoints::PostReposownerrepoTransfer(..) => Methods::Post,
  EndPoints::GetReposownerrepoVulnerabilityAlerts(..) => Methods::Get,
  EndPoints::PutReposownerrepoVulnerabilityAlerts(..) => Methods::Put,
  EndPoints::DeleteReposownerrepoVulnerabilityAlerts(..) => Methods::Delete,
  EndPoints::GetReposownerrepoZipballref(..) => Methods::Get,
  EndPoints::PostRepostemplateOwnertemplateRepoGenerate(..) => Methods::Post,
  EndPoints::GetRepositories(..) => Methods::Get,
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(..) => Methods::Get,
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(..) => Methods::Get,
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(..) => Methods::Get,
  EndPoints::PutRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(..) => Methods::Put,
  EndPoints::DeleteRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(..) => Methods::Delete,
  EndPoints::GetScimV2EnterprisesenterpriseGroups(..) => Methods::Get,
  EndPoints::PostScimV2EnterprisesenterpriseGroups(..) => Methods::Post,
  EndPoints::GetScimV2EnterprisesenterpriseGroupsscimGroupId(..) => Methods::Get,
  EndPoints::PutScimV2EnterprisesenterpriseGroupsscimGroupId(..) => Methods::Put,
  EndPoints::PatchScimV2EnterprisesenterpriseGroupsscimGroupId(..) => Methods::Patch,
  EndPoints::DeleteScimV2EnterprisesenterpriseGroupsscimGroupId(..) => Methods::Delete,
  EndPoints::GetScimV2EnterprisesenterpriseUsers(..) => Methods::Get,
  EndPoints::PostScimV2EnterprisesenterpriseUsers(..) => Methods::Post,
  EndPoints::GetScimV2EnterprisesenterpriseUsersscimUserId(..) => Methods::Get,
  EndPoints::PutScimV2EnterprisesenterpriseUsersscimUserId(..) => Methods::Put,
  EndPoints::PatchScimV2EnterprisesenterpriseUsersscimUserId(..) => Methods::Patch,
  EndPoints::DeleteScimV2EnterprisesenterpriseUsersscimUserId(..) => Methods::Delete,
  EndPoints::GetScimV2OrganizationsorgUsers(..) => Methods::Get,
  EndPoints::PostScimV2OrganizationsorgUsers(..) => Methods::Post,
  EndPoints::GetScimV2OrganizationsorgUsersscimUserId(..) => Methods::Get,
  EndPoints::PutScimV2OrganizationsorgUsersscimUserId(..) => Methods::Put,
  EndPoints::PatchScimV2OrganizationsorgUsersscimUserId(..) => Methods::Patch,
  EndPoints::DeleteScimV2OrganizationsorgUsersscimUserId(..) => Methods::Delete,
  EndPoints::GetSearchCode(..) => Methods::Get,
  EndPoints::GetSearchCommits(..) => Methods::Get,
  EndPoints::GetSearchIssues(..) => Methods::Get,
  EndPoints::GetSearchLabels(..) => Methods::Get,
  EndPoints::GetSearchRepositories(..) => Methods::Get,
  EndPoints::GetSearchTopics(..) => Methods::Get,
  EndPoints::GetSearchUsers(..) => Methods::Get,
  EndPoints::GetTeamsteamId(..) => Methods::Get,
  EndPoints::PatchTeamsteamId(..) => Methods::Patch,
  EndPoints::DeleteTeamsteamId(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdDiscussions(..) => Methods::Get,
  EndPoints::PostTeamsteamIdDiscussions(..) => Methods::Post,
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumber(..) => Methods::Get,
  EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumber(..) => Methods::Patch,
  EndPoints::DeleteTeamsteamIdDiscussionsdiscussionNumber(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberComments(..) => Methods::Get,
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberComments(..) => Methods::Post,
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Get,
  EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Patch,
  EndPoints::DeleteTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(..) => Methods::Get,
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(..) => Methods::Post,
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberReactions(..) => Methods::Get,
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberReactions(..) => Methods::Post,
  EndPoints::GetTeamsteamIdInvitations(..) => Methods::Get,
  EndPoints::GetTeamsteamIdMembers(..) => Methods::Get,
  EndPoints::GetTeamsteamIdMembersusername(..) => Methods::Get,
  EndPoints::PutTeamsteamIdMembersusername(..) => Methods::Put,
  EndPoints::DeleteTeamsteamIdMembersusername(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdMembershipsusername(..) => Methods::Get,
  EndPoints::PutTeamsteamIdMembershipsusername(..) => Methods::Put,
  EndPoints::DeleteTeamsteamIdMembershipsusername(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdProjects(..) => Methods::Get,
  EndPoints::GetTeamsteamIdProjectsprojectId(..) => Methods::Get,
  EndPoints::PutTeamsteamIdProjectsprojectId(..) => Methods::Put,
  EndPoints::DeleteTeamsteamIdProjectsprojectId(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdRepos(..) => Methods::Get,
  EndPoints::GetTeamsteamIdReposownerrepo(..) => Methods::Get,
  EndPoints::PutTeamsteamIdReposownerrepo(..) => Methods::Put,
  EndPoints::DeleteTeamsteamIdReposownerrepo(..) => Methods::Delete,
  EndPoints::GetTeamsteamIdTeamSyncGroupMappings(..) => Methods::Get,
  EndPoints::PatchTeamsteamIdTeamSyncGroupMappings(..) => Methods::Patch,
  EndPoints::GetTeamsteamIdTeams(..) => Methods::Get,
  EndPoints::GetUser(..) => Methods::Get,
  EndPoints::PatchUser(..) => Methods::Patch,
  EndPoints::GetUserBlocks(..) => Methods::Get,
  EndPoints::GetUserBlocksusername(..) => Methods::Get,
  EndPoints::PutUserBlocksusername(..) => Methods::Put,
  EndPoints::DeleteUserBlocksusername(..) => Methods::Delete,
  EndPoints::GetUserCodespaces(..) => Methods::Get,
  EndPoints::PostUserCodespaces(..) => Methods::Post,
  EndPoints::GetUserCodespacesSecrets(..) => Methods::Get,
  EndPoints::GetUserCodespacesSecretsPublicKey(..) => Methods::Get,
  EndPoints::GetUserCodespacesSecretssecretName(..) => Methods::Get,
  EndPoints::PutUserCodespacesSecretssecretName(..) => Methods::Put,
  EndPoints::DeleteUserCodespacesSecretssecretName(..) => Methods::Delete,
  EndPoints::GetUserCodespacesSecretssecretNameRepositories(..) => Methods::Get,
  EndPoints::PutUserCodespacesSecretssecretNameRepositories(..) => Methods::Put,
  EndPoints::PutUserCodespacesSecretssecretNameRepositoriesrepositoryId(..) => Methods::Put,
  EndPoints::DeleteUserCodespacesSecretssecretNameRepositoriesrepositoryId(..) => Methods::Delete,
  EndPoints::GetUserCodespacescodespaceName(..) => Methods::Get,
  EndPoints::PatchUserCodespacescodespaceName(..) => Methods::Patch,
  EndPoints::DeleteUserCodespacescodespaceName(..) => Methods::Delete,
  EndPoints::GetUserCodespacescodespaceNameMachines(..) => Methods::Get,
  EndPoints::PostUserCodespacescodespaceNameStart(..) => Methods::Post,
  EndPoints::PostUserCodespacescodespaceNameStop(..) => Methods::Post,
  EndPoints::PatchUserEmailVisibility(..) => Methods::Patch,
  EndPoints::GetUserEmails(..) => Methods::Get,
  EndPoints::PostUserEmails(..) => Methods::Post,
  EndPoints::DeleteUserEmails(..) => Methods::Delete,
  EndPoints::GetUserFollowers(..) => Methods::Get,
  EndPoints::GetUserFollowing(..) => Methods::Get,
  EndPoints::GetUserFollowingusername(..) => Methods::Get,
  EndPoints::PutUserFollowingusername(..) => Methods::Put,
  EndPoints::DeleteUserFollowingusername(..) => Methods::Delete,
  EndPoints::GetUserGpgKeys(..) => Methods::Get,
  EndPoints::PostUserGpgKeys(..) => Methods::Post,
  EndPoints::GetUserGpgKeysgpgKeyId(..) => Methods::Get,
  EndPoints::DeleteUserGpgKeysgpgKeyId(..) => Methods::Delete,
  EndPoints::GetUserInstallations(..) => Methods::Get,
  EndPoints::GetUserInstallationsinstallationIdRepositories(..) => Methods::Get,
  EndPoints::PutUserInstallationsinstallationIdRepositoriesrepositoryId(..) => Methods::Put,
  EndPoints::DeleteUserInstallationsinstallationIdRepositoriesrepositoryId(..) => Methods::Delete,
  EndPoints::GetUserInteractionLimits(..) => Methods::Get,
  EndPoints::PutUserInteractionLimits(..) => Methods::Put,
  EndPoints::DeleteUserInteractionLimits(..) => Methods::Delete,
  EndPoints::GetUserIssues(..) => Methods::Get,
  EndPoints::GetUserKeys(..) => Methods::Get,
  EndPoints::PostUserKeys(..) => Methods::Post,
  EndPoints::GetUserKeyskeyId(..) => Methods::Get,
  EndPoints::DeleteUserKeyskeyId(..) => Methods::Delete,
  EndPoints::GetUserMarketplacePurchases(..) => Methods::Get,
  EndPoints::GetUserMarketplacePurchasesStubbed(..) => Methods::Get,
  EndPoints::GetUserMembershipsOrgs(..) => Methods::Get,
  EndPoints::GetUserMembershipsOrgsorg(..) => Methods::Get,
  EndPoints::PatchUserMembershipsOrgsorg(..) => Methods::Patch,
  EndPoints::GetUserMigrations(..) => Methods::Get,
  EndPoints::PostUserMigrations(..) => Methods::Post,
  EndPoints::GetUserMigrationsmigrationId(..) => Methods::Get,
  EndPoints::GetUserMigrationsmigrationIdArchive(..) => Methods::Get,
  EndPoints::DeleteUserMigrationsmigrationIdArchive(..) => Methods::Delete,
  EndPoints::DeleteUserMigrationsmigrationIdReposrepoNameLock(..) => Methods::Delete,
  EndPoints::GetUserMigrationsmigrationIdRepositories(..) => Methods::Get,
  EndPoints::GetUserOrgs(..) => Methods::Get,
  EndPoints::GetUserPackages(..) => Methods::Get,
  EndPoints::GetUserPackagespackageTypepackageName(..) => Methods::Get,
  EndPoints::DeleteUserPackagespackageTypepackageName(..) => Methods::Delete,
  EndPoints::PostUserPackagespackageTypepackageNameRestore(..) => Methods::Post,
  EndPoints::GetUserPackagespackageTypepackageNameVersions(..) => Methods::Get,
  EndPoints::GetUserPackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Get,
  EndPoints::DeleteUserPackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Delete,
  EndPoints::PostUserPackagespackageTypepackageNameVersionspackageVersionIdRestore(..) => Methods::Post,
  EndPoints::PostUserProjects(..) => Methods::Post,
  EndPoints::GetUserPublicEmails(..) => Methods::Get,
  EndPoints::GetUserRepos(..) => Methods::Get,
  EndPoints::PostUserRepos(..) => Methods::Post,
  EndPoints::GetUserRepositoryInvitations(..) => Methods::Get,
  EndPoints::PatchUserRepositoryInvitationsinvitationId(..) => Methods::Patch,
  EndPoints::DeleteUserRepositoryInvitationsinvitationId(..) => Methods::Delete,
  EndPoints::GetUserStarred(..) => Methods::Get,
  EndPoints::GetUserStarredownerrepo(..) => Methods::Get,
  EndPoints::PutUserStarredownerrepo(..) => Methods::Put,
  EndPoints::DeleteUserStarredownerrepo(..) => Methods::Delete,
  EndPoints::GetUserSubscriptions(..) => Methods::Get,
  EndPoints::GetUserTeams(..) => Methods::Get,
  EndPoints::GetUsers(..) => Methods::Get,
  EndPoints::GetUsersusername(..) => Methods::Get,
  EndPoints::GetUsersusernameEvents(..) => Methods::Get,
  EndPoints::GetUsersusernameEventsOrgsorg(..) => Methods::Get,
  EndPoints::GetUsersusernameEventsPublic(..) => Methods::Get,
  EndPoints::GetUsersusernameFollowers(..) => Methods::Get,
  EndPoints::GetUsersusernameFollowing(..) => Methods::Get,
  EndPoints::GetUsersusernameFollowingtargetUser(..) => Methods::Get,
  EndPoints::GetUsersusernameGists(..) => Methods::Get,
  EndPoints::GetUsersusernameGpgKeys(..) => Methods::Get,
  EndPoints::GetUsersusernameHovercard(..) => Methods::Get,
  EndPoints::GetUsersusernameInstallation(..) => Methods::Get,
  EndPoints::GetUsersusernameKeys(..) => Methods::Get,
  EndPoints::GetUsersusernameOrgs(..) => Methods::Get,
  EndPoints::GetUsersusernamePackages(..) => Methods::Get,
  EndPoints::GetUsersusernamePackagespackageTypepackageName(..) => Methods::Get,
  EndPoints::DeleteUsersusernamePackagespackageTypepackageName(..) => Methods::Delete,
  EndPoints::PostUsersusernamePackagespackageTypepackageNameRestore(..) => Methods::Post,
  EndPoints::GetUsersusernamePackagespackageTypepackageNameVersions(..) => Methods::Get,
  EndPoints::GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Get,
  EndPoints::DeleteUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(..) => Methods::Delete,
  EndPoints::PostUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdRestore(..) => Methods::Post,
  EndPoints::GetUsersusernameProjects(..) => Methods::Get,
  EndPoints::GetUsersusernameReceivedEvents(..) => Methods::Get,
  EndPoints::GetUsersusernameReceivedEventsPublic(..) => Methods::Get,
  EndPoints::GetUsersusernameRepos(..) => Methods::Get,
  EndPoints::GetUsersusernameSettingsBillingActions(..) => Methods::Get,
  EndPoints::GetUsersusernameSettingsBillingPackages(..) => Methods::Get,
  EndPoints::GetUsersusernameSettingsBillingSharedStorage(..) => Methods::Get,
  EndPoints::GetUsersusernameStarred(..) => Methods::Get,
  EndPoints::GetUsersusernameSubscriptions(..) => Methods::Get,
  EndPoints::GetZen(..) => Methods::Get}
    }
    pub fn path(&self) -> String {
        match self { EndPoints::Get() => format!("/", ),
  EndPoints::GetApp() => format!("/app", ),
  EndPoints::PostAppManifestscodeConversions(code) => format!("/app-manifests/{code}/conversions",code = code ),
  EndPoints::GetAppHookConfig() => format!("/app/hook/config", ),
  EndPoints::PatchAppHookConfig() => format!("/app/hook/config", ),
  EndPoints::GetAppHookDeliveries() => format!("/app/hook/deliveries", ),
  EndPoints::GetAppHookDeliveriesdeliveryId(delivery_id) => format!("/app/hook/deliveries/{delivery_id}",delivery_id = delivery_id ),
  EndPoints::PostAppHookDeliveriesdeliveryIdAttempts(delivery_id) => format!("/app/hook/deliveries/{delivery_id}/attempts",delivery_id = delivery_id ),
  EndPoints::GetAppInstallations() => format!("/app/installations", ),
  EndPoints::GetAppInstallationsinstallationId(installation_id) => format!("/app/installations/{installation_id}",installation_id = installation_id ),
  EndPoints::DeleteAppInstallationsinstallationId(installation_id) => format!("/app/installations/{installation_id}",installation_id = installation_id ),
  EndPoints::PostAppInstallationsinstallationIdAccessTokens(installation_id) => format!("/app/installations/{installation_id}/access_tokens",installation_id = installation_id ),
  EndPoints::PutAppInstallationsinstallationIdSuspended(installation_id) => format!("/app/installations/{installation_id}/suspended",installation_id = installation_id ),
  EndPoints::DeleteAppInstallationsinstallationIdSuspended(installation_id) => format!("/app/installations/{installation_id}/suspended",installation_id = installation_id ),
  EndPoints::GetApplicationsGrants() => format!("/applications/grants", ),
  EndPoints::GetApplicationsGrantsgrantId(grant_id) => format!("/applications/grants/{grant_id}",grant_id = grant_id ),
  EndPoints::DeleteApplicationsGrantsgrantId(grant_id) => format!("/applications/grants/{grant_id}",grant_id = grant_id ),
  EndPoints::DeleteApplicationsclientIdGrant(client_id) => format!("/applications/{client_id}/grant",client_id = client_id ),
  EndPoints::PostApplicationsclientIdToken(client_id) => format!("/applications/{client_id}/token",client_id = client_id ),
  EndPoints::PatchApplicationsclientIdToken(client_id) => format!("/applications/{client_id}/token",client_id = client_id ),
  EndPoints::DeleteApplicationsclientIdToken(client_id) => format!("/applications/{client_id}/token",client_id = client_id ),
  EndPoints::PostApplicationsclientIdTokenScoped(client_id) => format!("/applications/{client_id}/token/scoped",client_id = client_id ),
  EndPoints::GetAppsappSlug(app_slug) => format!("/apps/{app_slug}",app_slug = app_slug ),
  EndPoints::GetAuthorizations() => format!("/authorizations", ),
  EndPoints::PostAuthorizations() => format!("/authorizations", ),
  EndPoints::PutAuthorizationsClientsclientId(client_id) => format!("/authorizations/clients/{client_id}",client_id = client_id ),
  EndPoints::PutAuthorizationsClientsclientIdfingerprint(client_id,fingerprint) => format!("/authorizations/clients/{client_id}/{fingerprint}",client_id = client_id,fingerprint = fingerprint ),
  EndPoints::GetAuthorizationsauthorizationId(authorization_id) => format!("/authorizations/{authorization_id}",authorization_id = authorization_id ),
  EndPoints::PatchAuthorizationsauthorizationId(authorization_id) => format!("/authorizations/{authorization_id}",authorization_id = authorization_id ),
  EndPoints::DeleteAuthorizationsauthorizationId(authorization_id) => format!("/authorizations/{authorization_id}",authorization_id = authorization_id ),
  EndPoints::GetCodesOfConduct() => format!("/codes_of_conduct", ),
  EndPoints::GetCodesOfConductkey(key) => format!("/codes_of_conduct/{key}",key = key ),
  EndPoints::GetEmojis() => format!("/emojis", ),
  EndPoints::GetEnterprisesenterpriseActionsPermissions(enterprise) => format!("/enterprises/{enterprise}/actions/permissions",enterprise = enterprise ),
  EndPoints::PutEnterprisesenterpriseActionsPermissions(enterprise) => format!("/enterprises/{enterprise}/actions/permissions",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseActionsPermissionsOrganizations(enterprise) => format!("/enterprises/{enterprise}/actions/permissions/organizations",enterprise = enterprise ),
  EndPoints::PutEnterprisesenterpriseActionsPermissionsOrganizations(enterprise) => format!("/enterprises/{enterprise}/actions/permissions/organizations",enterprise = enterprise ),
  EndPoints::PutEnterprisesenterpriseActionsPermissionsOrganizationsorgId(enterprise,org_id) => format!("/enterprises/{enterprise}/actions/permissions/organizations/{org_id}",enterprise = enterprise,org_id = org_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsPermissionsOrganizationsorgId(enterprise,org_id) => format!("/enterprises/{enterprise}/actions/permissions/organizations/{org_id}",enterprise = enterprise,org_id = org_id ),
  EndPoints::GetEnterprisesenterpriseActionsPermissionsSelectedActions(enterprise) => format!("/enterprises/{enterprise}/actions/permissions/selected-actions",enterprise = enterprise ),
  EndPoints::PutEnterprisesenterpriseActionsPermissionsSelectedActions(enterprise) => format!("/enterprises/{enterprise}/actions/permissions/selected-actions",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroups(enterprise) => format!("/enterprises/{enterprise}/actions/runner-groups",enterprise = enterprise ),
  EndPoints::PostEnterprisesenterpriseActionsRunnerGroups(enterprise) => format!("/enterprises/{enterprise}/actions/runner-groups",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(enterprise,runner_group_id,org_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}",enterprise = enterprise,runner_group_id = runner_group_id,org_id = org_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsorgId(enterprise,runner_group_id,org_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}",enterprise = enterprise,runner_group_id = runner_group_id,org_id = org_id ),
  EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(enterprise,runner_group_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners",enterprise = enterprise,runner_group_id = runner_group_id ),
  EndPoints::PutEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(enterprise,runner_group_id,runner_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}",enterprise = enterprise,runner_group_id = runner_group_id,runner_id = runner_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(enterprise,runner_group_id,runner_id) => format!("/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}",enterprise = enterprise,runner_group_id = runner_group_id,runner_id = runner_id ),
  EndPoints::GetEnterprisesenterpriseActionsRunners(enterprise) => format!("/enterprises/{enterprise}/actions/runners",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseActionsRunnersDownloads(enterprise) => format!("/enterprises/{enterprise}/actions/runners/downloads",enterprise = enterprise ),
  EndPoints::PostEnterprisesenterpriseActionsRunnersRegistrationToken(enterprise) => format!("/enterprises/{enterprise}/actions/runners/registration-token",enterprise = enterprise ),
  EndPoints::PostEnterprisesenterpriseActionsRunnersRemoveToken(enterprise) => format!("/enterprises/{enterprise}/actions/runners/remove-token",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerId(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerId(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}/labels",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}/labels",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}/labels",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise,runner_id) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}/labels",enterprise = enterprise,runner_id = runner_id ),
  EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(enterprise,runner_id,name) => format!("/enterprises/{enterprise}/actions/runners/{runner_id}/labels/{name}",enterprise = enterprise,runner_id = runner_id,name = name ),
  EndPoints::GetEnterprisesenterpriseAuditLog(enterprise) => format!("/enterprises/{enterprise}/audit-log",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseSettingsBillingActions(enterprise) => format!("/enterprises/{enterprise}/settings/billing/actions",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(enterprise) => format!("/enterprises/{enterprise}/settings/billing/advanced-security",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseSettingsBillingPackages(enterprise) => format!("/enterprises/{enterprise}/settings/billing/packages",enterprise = enterprise ),
  EndPoints::GetEnterprisesenterpriseSettingsBillingSharedStorage(enterprise) => format!("/enterprises/{enterprise}/settings/billing/shared-storage",enterprise = enterprise ),
  EndPoints::GetEvents() => format!("/events", ),
  EndPoints::GetFeeds() => format!("/feeds", ),
  EndPoints::GetGists() => format!("/gists", ),
  EndPoints::PostGists() => format!("/gists", ),
  EndPoints::GetGistsPublic() => format!("/gists/public", ),
  EndPoints::GetGistsStarred() => format!("/gists/starred", ),
  EndPoints::GetGistsgistId(gist_id) => format!("/gists/{gist_id}",gist_id = gist_id ),
  EndPoints::PatchGistsgistId(gist_id) => format!("/gists/{gist_id}",gist_id = gist_id ),
  EndPoints::DeleteGistsgistId(gist_id) => format!("/gists/{gist_id}",gist_id = gist_id ),
  EndPoints::GetGistsgistIdComments(gist_id) => format!("/gists/{gist_id}/comments",gist_id = gist_id ),
  EndPoints::PostGistsgistIdComments(gist_id) => format!("/gists/{gist_id}/comments",gist_id = gist_id ),
  EndPoints::GetGistsgistIdCommentscommentId(gist_id,comment_id) => format!("/gists/{gist_id}/comments/{comment_id}",gist_id = gist_id,comment_id = comment_id ),
  EndPoints::PatchGistsgistIdCommentscommentId(gist_id,comment_id) => format!("/gists/{gist_id}/comments/{comment_id}",gist_id = gist_id,comment_id = comment_id ),
  EndPoints::DeleteGistsgistIdCommentscommentId(gist_id,comment_id) => format!("/gists/{gist_id}/comments/{comment_id}",gist_id = gist_id,comment_id = comment_id ),
  EndPoints::GetGistsgistIdCommits(gist_id) => format!("/gists/{gist_id}/commits",gist_id = gist_id ),
  EndPoints::GetGistsgistIdForks(gist_id) => format!("/gists/{gist_id}/forks",gist_id = gist_id ),
  EndPoints::PostGistsgistIdForks(gist_id) => format!("/gists/{gist_id}/forks",gist_id = gist_id ),
  EndPoints::GetGistsgistIdStar(gist_id) => format!("/gists/{gist_id}/star",gist_id = gist_id ),
  EndPoints::PutGistsgistIdStar(gist_id) => format!("/gists/{gist_id}/star",gist_id = gist_id ),
  EndPoints::DeleteGistsgistIdStar(gist_id) => format!("/gists/{gist_id}/star",gist_id = gist_id ),
  EndPoints::GetGistsgistIdsha(gist_id,sha) => format!("/gists/{gist_id}/{sha}",gist_id = gist_id,sha = sha ),
  EndPoints::GetGitignoreTemplates() => format!("/gitignore/templates", ),
  EndPoints::GetGitignoreTemplatesname(name) => format!("/gitignore/templates/{name}",name = name ),
  EndPoints::GetInstallationRepositories() => format!("/installation/repositories", ),
  EndPoints::DeleteInstallationToken() => format!("/installation/token", ),
  EndPoints::GetIssues() => format!("/issues", ),
  EndPoints::GetLicenses() => format!("/licenses", ),
  EndPoints::GetLicenseslicense(license) => format!("/licenses/{license}",license = license ),
  EndPoints::PostMarkdown() => format!("/markdown", ),
  EndPoints::PostMarkdownRaw() => format!("/markdown/raw", ),
  EndPoints::GetMarketplaceListingAccountsaccountId(account_id) => format!("/marketplace_listing/accounts/{account_id}",account_id = account_id ),
  EndPoints::GetMarketplaceListingPlans() => format!("/marketplace_listing/plans", ),
  EndPoints::GetMarketplaceListingPlansplanIdAccounts(plan_id) => format!("/marketplace_listing/plans/{plan_id}/accounts",plan_id = plan_id ),
  EndPoints::GetMarketplaceListingStubbedAccountsaccountId(account_id) => format!("/marketplace_listing/stubbed/accounts/{account_id}",account_id = account_id ),
  EndPoints::GetMarketplaceListingStubbedPlans() => format!("/marketplace_listing/stubbed/plans", ),
  EndPoints::GetMarketplaceListingStubbedPlansplanIdAccounts(plan_id) => format!("/marketplace_listing/stubbed/plans/{plan_id}/accounts",plan_id = plan_id ),
  EndPoints::GetMeta() => format!("/meta", ),
  EndPoints::GetNetworksownerrepoEvents(owner,repo) => format!("/networks/{owner}/{repo}/events",owner = owner,repo = repo ),
  EndPoints::GetNotifications() => format!("/notifications", ),
  EndPoints::PutNotifications() => format!("/notifications", ),
  EndPoints::GetNotificationsThreadsthreadId(thread_id) => format!("/notifications/threads/{thread_id}",thread_id = thread_id ),
  EndPoints::PatchNotificationsThreadsthreadId(thread_id) => format!("/notifications/threads/{thread_id}",thread_id = thread_id ),
  EndPoints::GetNotificationsThreadsthreadIdSubscription(thread_id) => format!("/notifications/threads/{thread_id}/subscription",thread_id = thread_id ),
  EndPoints::PutNotificationsThreadsthreadIdSubscription(thread_id) => format!("/notifications/threads/{thread_id}/subscription",thread_id = thread_id ),
  EndPoints::DeleteNotificationsThreadsthreadIdSubscription(thread_id) => format!("/notifications/threads/{thread_id}/subscription",thread_id = thread_id ),
  EndPoints::GetOctocat() => format!("/octocat", ),
  EndPoints::GetOrganizations() => format!("/organizations", ),
  EndPoints::GetOrganizationsorganizationIdCustomRoles(organization_id) => format!("/organizations/{organization_id}/custom_roles",organization_id = organization_id ),
  EndPoints::GetOrgsorg(org) => format!("/orgs/{org}",org = org ),
  EndPoints::PatchOrgsorg(org) => format!("/orgs/{org}",org = org ),
  EndPoints::GetOrgsorgActionsPermissions(org) => format!("/orgs/{org}/actions/permissions",org = org ),
  EndPoints::PutOrgsorgActionsPermissions(org) => format!("/orgs/{org}/actions/permissions",org = org ),
  EndPoints::GetOrgsorgActionsPermissionsRepositories(org) => format!("/orgs/{org}/actions/permissions/repositories",org = org ),
  EndPoints::PutOrgsorgActionsPermissionsRepositories(org) => format!("/orgs/{org}/actions/permissions/repositories",org = org ),
  EndPoints::PutOrgsorgActionsPermissionsRepositoriesrepositoryId(org,repository_id) => format!("/orgs/{org}/actions/permissions/repositories/{repository_id}",org = org,repository_id = repository_id ),
  EndPoints::DeleteOrgsorgActionsPermissionsRepositoriesrepositoryId(org,repository_id) => format!("/orgs/{org}/actions/permissions/repositories/{repository_id}",org = org,repository_id = repository_id ),
  EndPoints::GetOrgsorgActionsPermissionsSelectedActions(org) => format!("/orgs/{org}/actions/permissions/selected-actions",org = org ),
  EndPoints::PutOrgsorgActionsPermissionsSelectedActions(org) => format!("/orgs/{org}/actions/permissions/selected-actions",org = org ),
  EndPoints::GetOrgsorgActionsRunnerGroups(org) => format!("/orgs/{org}/actions/runner-groups",org = org ),
  EndPoints::PostOrgsorgActionsRunnerGroups(org) => format!("/orgs/{org}/actions/runner-groups",org = org ),
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupId(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}",org = org,runner_group_id = runner_group_id ),
  EndPoints::PatchOrgsorgActionsRunnerGroupsrunnerGroupId(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}",org = org,runner_group_id = runner_group_id ),
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupId(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}",org = org,runner_group_id = runner_group_id ),
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories",org = org,runner_group_id = runner_group_id ),
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories",org = org,runner_group_id = runner_group_id ),
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(org,runner_group_id,repository_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}",org = org,runner_group_id = runner_group_id,repository_id = repository_id ),
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesrepositoryId(org,runner_group_id,repository_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}",org = org,runner_group_id = runner_group_id,repository_id = repository_id ),
  EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/runners",org = org,runner_group_id = runner_group_id ),
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(org,runner_group_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/runners",org = org,runner_group_id = runner_group_id ),
  EndPoints::PutOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(org,runner_group_id,runner_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}",org = org,runner_group_id = runner_group_id,runner_id = runner_id ),
  EndPoints::DeleteOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersrunnerId(org,runner_group_id,runner_id) => format!("/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}",org = org,runner_group_id = runner_group_id,runner_id = runner_id ),
  EndPoints::GetOrgsorgActionsRunners(org) => format!("/orgs/{org}/actions/runners",org = org ),
  EndPoints::GetOrgsorgActionsRunnersDownloads(org) => format!("/orgs/{org}/actions/runners/downloads",org = org ),
  EndPoints::PostOrgsorgActionsRunnersRegistrationToken(org) => format!("/orgs/{org}/actions/runners/registration-token",org = org ),
  EndPoints::PostOrgsorgActionsRunnersRemoveToken(org) => format!("/orgs/{org}/actions/runners/remove-token",org = org ),
  EndPoints::GetOrgsorgActionsRunnersrunnerId(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}",org = org,runner_id = runner_id ),
  EndPoints::DeleteOrgsorgActionsRunnersrunnerId(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}",org = org,runner_id = runner_id ),
  EndPoints::GetOrgsorgActionsRunnersrunnerIdLabels(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}/labels",org = org,runner_id = runner_id ),
  EndPoints::PostOrgsorgActionsRunnersrunnerIdLabels(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}/labels",org = org,runner_id = runner_id ),
  EndPoints::PutOrgsorgActionsRunnersrunnerIdLabels(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}/labels",org = org,runner_id = runner_id ),
  EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabels(org,runner_id) => format!("/orgs/{org}/actions/runners/{runner_id}/labels",org = org,runner_id = runner_id ),
  EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabelsname(org,runner_id,name) => format!("/orgs/{org}/actions/runners/{runner_id}/labels/{name}",org = org,runner_id = runner_id,name = name ),
  EndPoints::GetOrgsorgActionsSecrets(org) => format!("/orgs/{org}/actions/secrets",org = org ),
  EndPoints::GetOrgsorgActionsSecretsPublicKey(org) => format!("/orgs/{org}/actions/secrets/public-key",org = org ),
  EndPoints::GetOrgsorgActionsSecretssecretName(org,secret_name) => format!("/orgs/{org}/actions/secrets/{secret_name}",org = org,secret_name = secret_name ),
  EndPoints::PutOrgsorgActionsSecretssecretName(org,secret_name) => format!("/orgs/{org}/actions/secrets/{secret_name}",org = org,secret_name = secret_name ),
  EndPoints::DeleteOrgsorgActionsSecretssecretName(org,secret_name) => format!("/orgs/{org}/actions/secrets/{secret_name}",org = org,secret_name = secret_name ),
  EndPoints::GetOrgsorgActionsSecretssecretNameRepositories(org,secret_name) => format!("/orgs/{org}/actions/secrets/{secret_name}/repositories",org = org,secret_name = secret_name ),
  EndPoints::PutOrgsorgActionsSecretssecretNameRepositories(org,secret_name) => format!("/orgs/{org}/actions/secrets/{secret_name}/repositories",org = org,secret_name = secret_name ),
  EndPoints::PutOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(org,secret_name,repository_id) => format!("/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",org = org,secret_name = secret_name,repository_id = repository_id ),
  EndPoints::DeleteOrgsorgActionsSecretssecretNameRepositoriesrepositoryId(org,secret_name,repository_id) => format!("/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",org = org,secret_name = secret_name,repository_id = repository_id ),
  EndPoints::GetOrgsorgAuditLog(org) => format!("/orgs/{org}/audit-log",org = org ),
  EndPoints::GetOrgsorgBlocks(org) => format!("/orgs/{org}/blocks",org = org ),
  EndPoints::GetOrgsorgBlocksusername(org,username) => format!("/orgs/{org}/blocks/{username}",org = org,username = username ),
  EndPoints::PutOrgsorgBlocksusername(org,username) => format!("/orgs/{org}/blocks/{username}",org = org,username = username ),
  EndPoints::DeleteOrgsorgBlocksusername(org,username) => format!("/orgs/{org}/blocks/{username}",org = org,username = username ),
  EndPoints::GetOrgsorgCredentialAuthorizations(org) => format!("/orgs/{org}/credential-authorizations",org = org ),
  EndPoints::DeleteOrgsorgCredentialAuthorizationscredentialId(org,credential_id) => format!("/orgs/{org}/credential-authorizations/{credential_id}",org = org,credential_id = credential_id ),
  EndPoints::GetOrgsorgEvents(org) => format!("/orgs/{org}/events",org = org ),
  EndPoints::GetOrgsorgExternalGroupgroupId(org,group_id) => format!("/orgs/{org}/external-group/{group_id}",org = org,group_id = group_id ),
  EndPoints::GetOrgsorgExternalGroups(org) => format!("/orgs/{org}/external-groups",org = org ),
  EndPoints::GetOrgsorgFailedInvitations(org) => format!("/orgs/{org}/failed_invitations",org = org ),
  EndPoints::GetOrgsorgHooks(org) => format!("/orgs/{org}/hooks",org = org ),
  EndPoints::PostOrgsorgHooks(org) => format!("/orgs/{org}/hooks",org = org ),
  EndPoints::GetOrgsorgHookshookId(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}",org = org,hook_id = hook_id ),
  EndPoints::PatchOrgsorgHookshookId(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}",org = org,hook_id = hook_id ),
  EndPoints::DeleteOrgsorgHookshookId(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}",org = org,hook_id = hook_id ),
  EndPoints::GetOrgsorgHookshookIdConfig(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}/config",org = org,hook_id = hook_id ),
  EndPoints::PatchOrgsorgHookshookIdConfig(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}/config",org = org,hook_id = hook_id ),
  EndPoints::GetOrgsorgHookshookIdDeliveries(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}/deliveries",org = org,hook_id = hook_id ),
  EndPoints::GetOrgsorgHookshookIdDeliveriesdeliveryId(org,hook_id,delivery_id) => format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}",org = org,hook_id = hook_id,delivery_id = delivery_id ),
  EndPoints::PostOrgsorgHookshookIdDeliveriesdeliveryIdAttempts(org,hook_id,delivery_id) => format!("/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts",org = org,hook_id = hook_id,delivery_id = delivery_id ),
  EndPoints::PostOrgsorgHookshookIdPings(org,hook_id) => format!("/orgs/{org}/hooks/{hook_id}/pings",org = org,hook_id = hook_id ),
  EndPoints::GetOrgsorgInstallation(org) => format!("/orgs/{org}/installation",org = org ),
  EndPoints::GetOrgsorgInstallations(org) => format!("/orgs/{org}/installations",org = org ),
  EndPoints::GetOrgsorgInteractionLimits(org) => format!("/orgs/{org}/interaction-limits",org = org ),
  EndPoints::PutOrgsorgInteractionLimits(org) => format!("/orgs/{org}/interaction-limits",org = org ),
  EndPoints::DeleteOrgsorgInteractionLimits(org) => format!("/orgs/{org}/interaction-limits",org = org ),
  EndPoints::GetOrgsorgInvitations(org) => format!("/orgs/{org}/invitations",org = org ),
  EndPoints::PostOrgsorgInvitations(org) => format!("/orgs/{org}/invitations",org = org ),
  EndPoints::DeleteOrgsorgInvitationsinvitationId(org,invitation_id) => format!("/orgs/{org}/invitations/{invitation_id}",org = org,invitation_id = invitation_id ),
  EndPoints::GetOrgsorgInvitationsinvitationIdTeams(org,invitation_id) => format!("/orgs/{org}/invitations/{invitation_id}/teams",org = org,invitation_id = invitation_id ),
  EndPoints::GetOrgsorgIssues(org) => format!("/orgs/{org}/issues",org = org ),
  EndPoints::GetOrgsorgMembers(org) => format!("/orgs/{org}/members",org = org ),
  EndPoints::GetOrgsorgMembersusername(org,username) => format!("/orgs/{org}/members/{username}",org = org,username = username ),
  EndPoints::DeleteOrgsorgMembersusername(org,username) => format!("/orgs/{org}/members/{username}",org = org,username = username ),
  EndPoints::GetOrgsorgMembershipsusername(org,username) => format!("/orgs/{org}/memberships/{username}",org = org,username = username ),
  EndPoints::PutOrgsorgMembershipsusername(org,username) => format!("/orgs/{org}/memberships/{username}",org = org,username = username ),
  EndPoints::DeleteOrgsorgMembershipsusername(org,username) => format!("/orgs/{org}/memberships/{username}",org = org,username = username ),
  EndPoints::GetOrgsorgMigrations(org) => format!("/orgs/{org}/migrations",org = org ),
  EndPoints::PostOrgsorgMigrations(org) => format!("/orgs/{org}/migrations",org = org ),
  EndPoints::GetOrgsorgMigrationsmigrationId(org,migration_id) => format!("/orgs/{org}/migrations/{migration_id}",org = org,migration_id = migration_id ),
  EndPoints::GetOrgsorgMigrationsmigrationIdArchive(org,migration_id) => format!("/orgs/{org}/migrations/{migration_id}/archive",org = org,migration_id = migration_id ),
  EndPoints::DeleteOrgsorgMigrationsmigrationIdArchive(org,migration_id) => format!("/orgs/{org}/migrations/{migration_id}/archive",org = org,migration_id = migration_id ),
  EndPoints::DeleteOrgsorgMigrationsmigrationIdReposrepoNameLock(org,migration_id,repo_name) => format!("/orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock",org = org,migration_id = migration_id,repo_name = repo_name ),
  EndPoints::GetOrgsorgMigrationsmigrationIdRepositories(org,migration_id) => format!("/orgs/{org}/migrations/{migration_id}/repositories",org = org,migration_id = migration_id ),
  EndPoints::GetOrgsorgOutsideCollaborators(org) => format!("/orgs/{org}/outside_collaborators",org = org ),
  EndPoints::PutOrgsorgOutsideCollaboratorsusername(org,username) => format!("/orgs/{org}/outside_collaborators/{username}",org = org,username = username ),
  EndPoints::DeleteOrgsorgOutsideCollaboratorsusername(org,username) => format!("/orgs/{org}/outside_collaborators/{username}",org = org,username = username ),
  EndPoints::GetOrgsorgPackages(org) => format!("/orgs/{org}/packages",org = org ),
  EndPoints::GetOrgsorgPackagespackageTypepackageName(org,package_type,package_name) => format!("/orgs/{org}/packages/{package_type}/{package_name}",org = org,package_type = package_type,package_name = package_name ),
  EndPoints::DeleteOrgsorgPackagespackageTypepackageName(org,package_type,package_name) => format!("/orgs/{org}/packages/{package_type}/{package_name}",org = org,package_type = package_type,package_name = package_name ),
  EndPoints::PostOrgsorgPackagespackageTypepackageNameRestore(org,package_type,package_name) => format!("/orgs/{org}/packages/{package_type}/{package_name}/restore",org = org,package_type = package_type,package_name = package_name ),
  EndPoints::GetOrgsorgPackagespackageTypepackageNameVersions(org,package_type,package_name) => format!("/orgs/{org}/packages/{package_type}/{package_name}/versions",org = org,package_type = package_type,package_name = package_name ),
  EndPoints::GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(org,package_type,package_name,package_version_id) => format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}",org = org,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::DeleteOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(org,package_type,package_name,package_version_id) => format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}",org = org,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::PostOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdRestore(org,package_type,package_name,package_version_id) => format!("/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore",org = org,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::GetOrgsorgProjects(org) => format!("/orgs/{org}/projects",org = org ),
  EndPoints::PostOrgsorgProjects(org) => format!("/orgs/{org}/projects",org = org ),
  EndPoints::GetOrgsorgPublicMembers(org) => format!("/orgs/{org}/public_members",org = org ),
  EndPoints::GetOrgsorgPublicMembersusername(org,username) => format!("/orgs/{org}/public_members/{username}",org = org,username = username ),
  EndPoints::PutOrgsorgPublicMembersusername(org,username) => format!("/orgs/{org}/public_members/{username}",org = org,username = username ),
  EndPoints::DeleteOrgsorgPublicMembersusername(org,username) => format!("/orgs/{org}/public_members/{username}",org = org,username = username ),
  EndPoints::GetOrgsorgRepos(org) => format!("/orgs/{org}/repos",org = org ),
  EndPoints::PostOrgsorgRepos(org) => format!("/orgs/{org}/repos",org = org ),
  EndPoints::GetOrgsorgSecretScanningAlerts(org) => format!("/orgs/{org}/secret-scanning/alerts",org = org ),
  EndPoints::GetOrgsorgSettingsBillingActions(org) => format!("/orgs/{org}/settings/billing/actions",org = org ),
  EndPoints::GetOrgsorgSettingsBillingAdvancedSecurity(org) => format!("/orgs/{org}/settings/billing/advanced-security",org = org ),
  EndPoints::GetOrgsorgSettingsBillingPackages(org) => format!("/orgs/{org}/settings/billing/packages",org = org ),
  EndPoints::GetOrgsorgSettingsBillingSharedStorage(org) => format!("/orgs/{org}/settings/billing/shared-storage",org = org ),
  EndPoints::GetOrgsorgTeamSyncGroups(org) => format!("/orgs/{org}/team-sync/groups",org = org ),
  EndPoints::GetOrgsorgTeams(org) => format!("/orgs/{org}/teams",org = org ),
  EndPoints::PostOrgsorgTeams(org) => format!("/orgs/{org}/teams",org = org ),
  EndPoints::GetOrgsorgTeamsteamSlug(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}",org = org,team_slug = team_slug ),
  EndPoints::PatchOrgsorgTeamsteamSlug(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}",org = org,team_slug = team_slug ),
  EndPoints::DeleteOrgsorgTeamsteamSlug(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussions(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/discussions",org = org,team_slug = team_slug ),
  EndPoints::PostOrgsorgTeamsteamSlugDiscussions(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/discussions",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberComments(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(org,team_slug,discussion_number,comment_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(org,team_slug,discussion_number,comment_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(org,team_slug,discussion_number,comment_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(org,team_slug,discussion_number,comment_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(org,team_slug,discussion_number,comment_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsreactionId(org,team_slug,discussion_number,comment_number,reaction_id) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}",org = org,team_slug = team_slug,discussion_number = discussion_number,comment_number = comment_number,reaction_id = reaction_id ),
  EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(org,team_slug,discussion_number) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions",org = org,team_slug = team_slug,discussion_number = discussion_number ),
  EndPoints::DeleteOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsreactionId(org,team_slug,discussion_number,reaction_id) => format!("/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id}",org = org,team_slug = team_slug,discussion_number = discussion_number,reaction_id = reaction_id ),
  EndPoints::PatchOrgsorgTeamsteamSlugExternalGroups(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/external-groups",org = org,team_slug = team_slug ),
  EndPoints::DeleteOrgsorgTeamsteamSlugExternalGroups(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/external-groups",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugInvitations(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/invitations",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugMembers(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/members",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugMembershipsusername(org,team_slug,username) => format!("/orgs/{org}/teams/{team_slug}/memberships/{username}",org = org,team_slug = team_slug,username = username ),
  EndPoints::PutOrgsorgTeamsteamSlugMembershipsusername(org,team_slug,username) => format!("/orgs/{org}/teams/{team_slug}/memberships/{username}",org = org,team_slug = team_slug,username = username ),
  EndPoints::DeleteOrgsorgTeamsteamSlugMembershipsusername(org,team_slug,username) => format!("/orgs/{org}/teams/{team_slug}/memberships/{username}",org = org,team_slug = team_slug,username = username ),
  EndPoints::GetOrgsorgTeamsteamSlugProjects(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/projects",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugProjectsprojectId(org,team_slug,project_id) => format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}",org = org,team_slug = team_slug,project_id = project_id ),
  EndPoints::PutOrgsorgTeamsteamSlugProjectsprojectId(org,team_slug,project_id) => format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}",org = org,team_slug = team_slug,project_id = project_id ),
  EndPoints::DeleteOrgsorgTeamsteamSlugProjectsprojectId(org,team_slug,project_id) => format!("/orgs/{org}/teams/{team_slug}/projects/{project_id}",org = org,team_slug = team_slug,project_id = project_id ),
  EndPoints::GetOrgsorgTeamsteamSlugRepos(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/repos",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugReposownerrepo(org,team_slug,owner,repo) => format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}",org = org,team_slug = team_slug,owner = owner,repo = repo ),
  EndPoints::PutOrgsorgTeamsteamSlugReposownerrepo(org,team_slug,owner,repo) => format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}",org = org,team_slug = team_slug,owner = owner,repo = repo ),
  EndPoints::DeleteOrgsorgTeamsteamSlugReposownerrepo(org,team_slug,owner,repo) => format!("/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}",org = org,team_slug = team_slug,owner = owner,repo = repo ),
  EndPoints::GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/team-sync/group-mappings",org = org,team_slug = team_slug ),
  EndPoints::PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/team-sync/group-mappings",org = org,team_slug = team_slug ),
  EndPoints::GetOrgsorgTeamsteamSlugTeams(org,team_slug) => format!("/orgs/{org}/teams/{team_slug}/teams",org = org,team_slug = team_slug ),
  EndPoints::GetProjectsColumnsCardscardId(card_id) => format!("/projects/columns/cards/{card_id}",card_id = card_id ),
  EndPoints::PatchProjectsColumnsCardscardId(card_id) => format!("/projects/columns/cards/{card_id}",card_id = card_id ),
  EndPoints::DeleteProjectsColumnsCardscardId(card_id) => format!("/projects/columns/cards/{card_id}",card_id = card_id ),
  EndPoints::PostProjectsColumnsCardscardIdMoves(card_id) => format!("/projects/columns/cards/{card_id}/moves",card_id = card_id ),
  EndPoints::GetProjectsColumnscolumnId(column_id) => format!("/projects/columns/{column_id}",column_id = column_id ),
  EndPoints::PatchProjectsColumnscolumnId(column_id) => format!("/projects/columns/{column_id}",column_id = column_id ),
  EndPoints::DeleteProjectsColumnscolumnId(column_id) => format!("/projects/columns/{column_id}",column_id = column_id ),
  EndPoints::GetProjectsColumnscolumnIdCards(column_id) => format!("/projects/columns/{column_id}/cards",column_id = column_id ),
  EndPoints::PostProjectsColumnscolumnIdCards(column_id) => format!("/projects/columns/{column_id}/cards",column_id = column_id ),
  EndPoints::PostProjectsColumnscolumnIdMoves(column_id) => format!("/projects/columns/{column_id}/moves",column_id = column_id ),
  EndPoints::GetProjectsprojectId(project_id) => format!("/projects/{project_id}",project_id = project_id ),
  EndPoints::PatchProjectsprojectId(project_id) => format!("/projects/{project_id}",project_id = project_id ),
  EndPoints::DeleteProjectsprojectId(project_id) => format!("/projects/{project_id}",project_id = project_id ),
  EndPoints::GetProjectsprojectIdCollaborators(project_id) => format!("/projects/{project_id}/collaborators",project_id = project_id ),
  EndPoints::PutProjectsprojectIdCollaboratorsusername(project_id,username) => format!("/projects/{project_id}/collaborators/{username}",project_id = project_id,username = username ),
  EndPoints::DeleteProjectsprojectIdCollaboratorsusername(project_id,username) => format!("/projects/{project_id}/collaborators/{username}",project_id = project_id,username = username ),
  EndPoints::GetProjectsprojectIdCollaboratorsusernamePermission(project_id,username) => format!("/projects/{project_id}/collaborators/{username}/permission",project_id = project_id,username = username ),
  EndPoints::GetProjectsprojectIdColumns(project_id) => format!("/projects/{project_id}/columns",project_id = project_id ),
  EndPoints::PostProjectsprojectIdColumns(project_id) => format!("/projects/{project_id}/columns",project_id = project_id ),
  EndPoints::GetRateLimit() => format!("/rate_limit", ),
  EndPoints::DeleteReactionsreactionId(reaction_id) => format!("/reactions/{reaction_id}",reaction_id = reaction_id ),
  EndPoints::GetReposownerrepo(owner,repo) => format!("/repos/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepo(owner,repo) => format!("/repos/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepo(owner,repo) => format!("/repos/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsArtifacts(owner,repo) => format!("/repos/{owner}/{repo}/actions/artifacts",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsArtifactsartifactId(owner,repo,artifact_id) => format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}",owner = owner,repo = repo,artifact_id = artifact_id ),
  EndPoints::DeleteReposownerrepoActionsArtifactsartifactId(owner,repo,artifact_id) => format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}",owner = owner,repo = repo,artifact_id = artifact_id ),
  EndPoints::GetReposownerrepoActionsArtifactsartifactIdarchiveFormat(owner,repo,artifact_id,archive_format) => format!("/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}",owner = owner,repo = repo,artifact_id = artifact_id,archive_format = archive_format ),
  EndPoints::GetReposownerrepoActionsJobsjobId(owner,repo,job_id) => format!("/repos/{owner}/{repo}/actions/jobs/{job_id}",owner = owner,repo = repo,job_id = job_id ),
  EndPoints::GetReposownerrepoActionsJobsjobIdLogs(owner,repo,job_id) => format!("/repos/{owner}/{repo}/actions/jobs/{job_id}/logs",owner = owner,repo = repo,job_id = job_id ),
  EndPoints::GetReposownerrepoActionsPermissions(owner,repo) => format!("/repos/{owner}/{repo}/actions/permissions",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoActionsPermissions(owner,repo) => format!("/repos/{owner}/{repo}/actions/permissions",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsPermissionsSelectedActions(owner,repo) => format!("/repos/{owner}/{repo}/actions/permissions/selected-actions",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoActionsPermissionsSelectedActions(owner,repo) => format!("/repos/{owner}/{repo}/actions/permissions/selected-actions",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsRunners(owner,repo) => format!("/repos/{owner}/{repo}/actions/runners",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsRunnersDownloads(owner,repo) => format!("/repos/{owner}/{repo}/actions/runners/downloads",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoActionsRunnersRegistrationToken(owner,repo) => format!("/repos/{owner}/{repo}/actions/runners/registration-token",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoActionsRunnersRemoveToken(owner,repo) => format!("/repos/{owner}/{repo}/actions/runners/remove-token",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsRunnersrunnerId(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerId(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::GetReposownerrepoActionsRunnersrunnerIdLabels(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::PostReposownerrepoActionsRunnersrunnerIdLabels(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::PutReposownerrepoActionsRunnersrunnerIdLabels(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabels(owner,repo,runner_id) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels",owner = owner,repo = repo,runner_id = runner_id ),
  EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(owner,repo,runner_id,name) => format!("/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}",owner = owner,repo = repo,runner_id = runner_id,name = name ),
  EndPoints::GetReposownerrepoActionsRuns(owner,repo) => format!("/repos/{owner}/{repo}/actions/runs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsRunsrunId(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::DeleteReposownerrepoActionsRunsrunId(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdApprovals(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approvals",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::PostReposownerrepoActionsRunsrunIdApprove(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/approve",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdArtifacts(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(owner,repo,run_id,attempt_number) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}",owner = owner,repo = repo,run_id = run_id,attempt_number = attempt_number ),
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(owner,repo,run_id,attempt_number) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs",owner = owner,repo = repo,run_id = run_id,attempt_number = attempt_number ),
  EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberLogs(owner,repo,run_id,attempt_number) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs",owner = owner,repo = repo,run_id = run_id,attempt_number = attempt_number ),
  EndPoints::PostReposownerrepoActionsRunsrunIdCancel(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/cancel",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdJobs(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/jobs",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdLogs(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/logs",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::DeleteReposownerrepoActionsRunsrunIdLogs(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/logs",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdPendingDeployments(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::PostReposownerrepoActionsRunsrunIdPendingDeployments(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::PostReposownerrepoActionsRunsrunIdRerun(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/rerun",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsRunsrunIdTiming(owner,repo,run_id) => format!("/repos/{owner}/{repo}/actions/runs/{run_id}/timing",owner = owner,repo = repo,run_id = run_id ),
  EndPoints::GetReposownerrepoActionsSecrets(owner,repo) => format!("/repos/{owner}/{repo}/actions/secrets",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsSecretsPublicKey(owner,repo) => format!("/repos/{owner}/{repo}/actions/secrets/public-key",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsSecretssecretName(owner,repo,secret_name) => format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}",owner = owner,repo = repo,secret_name = secret_name ),
  EndPoints::PutReposownerrepoActionsSecretssecretName(owner,repo,secret_name) => format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}",owner = owner,repo = repo,secret_name = secret_name ),
  EndPoints::DeleteReposownerrepoActionsSecretssecretName(owner,repo,secret_name) => format!("/repos/{owner}/{repo}/actions/secrets/{secret_name}",owner = owner,repo = repo,secret_name = secret_name ),
  EndPoints::GetReposownerrepoActionsWorkflows(owner,repo) => format!("/repos/{owner}/{repo}/actions/workflows",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowId(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::PutReposownerrepoActionsWorkflowsworkflowIdDisable(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::PostReposownerrepoActionsWorkflowsworkflowIdDispatches(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::PutReposownerrepoActionsWorkflowsworkflowIdEnable(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdRuns(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdTiming(owner,repo,workflow_id) => format!("/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing",owner = owner,repo = repo,workflow_id = workflow_id ),
  EndPoints::GetReposownerrepoAssignees(owner,repo) => format!("/repos/{owner}/{repo}/assignees",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoAssigneesassignee(owner,repo,assignee) => format!("/repos/{owner}/{repo}/assignees/{assignee}",owner = owner,repo = repo,assignee = assignee ),
  EndPoints::GetReposownerrepoAutolinks(owner,repo) => format!("/repos/{owner}/{repo}/autolinks",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoAutolinks(owner,repo) => format!("/repos/{owner}/{repo}/autolinks",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoAutolinksautolinkId(owner,repo,autolink_id) => format!("/repos/{owner}/{repo}/autolinks/{autolink_id}",owner = owner,repo = repo,autolink_id = autolink_id ),
  EndPoints::DeleteReposownerrepoAutolinksautolinkId(owner,repo,autolink_id) => format!("/repos/{owner}/{repo}/autolinks/{autolink_id}",owner = owner,repo = repo,autolink_id = autolink_id ),
  EndPoints::PutReposownerrepoAutomatedSecurityFixes(owner,repo) => format!("/repos/{owner}/{repo}/automated-security-fixes",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoAutomatedSecurityFixes(owner,repo) => format!("/repos/{owner}/{repo}/automated-security-fixes",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoBranches(owner,repo) => format!("/repos/{owner}/{repo}/branches",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoBranchesbranch(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtection(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection",owner = owner,repo = repo,branch = branch ),
  EndPoints::PutReposownerrepoBranchesbranchProtection(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtection(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionEnforceAdmins(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionEnforceAdmins(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionEnforceAdmins(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews",owner = owner,repo = repo,branch = branch ),
  EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredSignatures(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredSignatures(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredSignatures(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks",owner = owner,repo = repo,branch = branch ),
  EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecks(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts",owner = owner,repo = repo,branch = branch ),
  EndPoints::PutReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRequiredStatusChecksContexts(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictions(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictions(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsApps(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsApps(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps",owner = owner,repo = repo,branch = branch ),
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsApps(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsApps(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsTeams(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsTeams(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams",owner = owner,repo = repo,branch = branch ),
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsTeams(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsTeams(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams",owner = owner,repo = repo,branch = branch ),
  EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictionsUsers(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchProtectionRestrictionsUsers(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users",owner = owner,repo = repo,branch = branch ),
  EndPoints::PutReposownerrepoBranchesbranchProtectionRestrictionsUsers(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users",owner = owner,repo = repo,branch = branch ),
  EndPoints::DeleteReposownerrepoBranchesbranchProtectionRestrictionsUsers(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoBranchesbranchRename(owner,repo,branch) => format!("/repos/{owner}/{repo}/branches/{branch}/rename",owner = owner,repo = repo,branch = branch ),
  EndPoints::PostReposownerrepoCheckRuns(owner,repo) => format!("/repos/{owner}/{repo}/check-runs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCheckRunscheckRunId(owner,repo,check_run_id) => format!("/repos/{owner}/{repo}/check-runs/{check_run_id}",owner = owner,repo = repo,check_run_id = check_run_id ),
  EndPoints::PatchReposownerrepoCheckRunscheckRunId(owner,repo,check_run_id) => format!("/repos/{owner}/{repo}/check-runs/{check_run_id}",owner = owner,repo = repo,check_run_id = check_run_id ),
  EndPoints::GetReposownerrepoCheckRunscheckRunIdAnnotations(owner,repo,check_run_id) => format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations",owner = owner,repo = repo,check_run_id = check_run_id ),
  EndPoints::PostReposownerrepoCheckRunscheckRunIdRerequest(owner,repo,check_run_id) => format!("/repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest",owner = owner,repo = repo,check_run_id = check_run_id ),
  EndPoints::PostReposownerrepoCheckSuites(owner,repo) => format!("/repos/{owner}/{repo}/check-suites",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoCheckSuitesPreferences(owner,repo) => format!("/repos/{owner}/{repo}/check-suites/preferences",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCheckSuitescheckSuiteId(owner,repo,check_suite_id) => format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}",owner = owner,repo = repo,check_suite_id = check_suite_id ),
  EndPoints::GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(owner,repo,check_suite_id) => format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs",owner = owner,repo = repo,check_suite_id = check_suite_id ),
  EndPoints::PostReposownerrepoCheckSuitescheckSuiteIdRerequest(owner,repo,check_suite_id) => format!("/repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest",owner = owner,repo = repo,check_suite_id = check_suite_id ),
  EndPoints::GetReposownerrepoCodeScanningAlerts(owner,repo) => format!("/repos/{owner}/{repo}/code-scanning/alerts",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCodeScanningAlertsalertNumber(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::PatchReposownerrepoCodeScanningAlertsalertNumber(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::GetReposownerrepoCodeScanningAlertsalertNumberInstances(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::GetReposownerrepoCodeScanningAnalyses(owner,repo) => format!("/repos/{owner}/{repo}/code-scanning/analyses",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCodeScanningAnalysesanalysisId(owner,repo,analysis_id) => format!("/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}",owner = owner,repo = repo,analysis_id = analysis_id ),
  EndPoints::DeleteReposownerrepoCodeScanningAnalysesanalysisId(owner,repo,analysis_id) => format!("/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}",owner = owner,repo = repo,analysis_id = analysis_id ),
  EndPoints::PostReposownerrepoCodeScanningSarifs(owner,repo) => format!("/repos/{owner}/{repo}/code-scanning/sarifs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCodeScanningSarifssarifId(owner,repo,sarif_id) => format!("/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}",owner = owner,repo = repo,sarif_id = sarif_id ),
  EndPoints::GetReposownerrepoCodespaces(owner,repo) => format!("/repos/{owner}/{repo}/codespaces",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoCodespaces(owner,repo) => format!("/repos/{owner}/{repo}/codespaces",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCodespacesMachines(owner,repo) => format!("/repos/{owner}/{repo}/codespaces/machines",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCollaborators(owner,repo) => format!("/repos/{owner}/{repo}/collaborators",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCollaboratorsusername(owner,repo,username) => format!("/repos/{owner}/{repo}/collaborators/{username}",owner = owner,repo = repo,username = username ),
  EndPoints::PutReposownerrepoCollaboratorsusername(owner,repo,username) => format!("/repos/{owner}/{repo}/collaborators/{username}",owner = owner,repo = repo,username = username ),
  EndPoints::DeleteReposownerrepoCollaboratorsusername(owner,repo,username) => format!("/repos/{owner}/{repo}/collaborators/{username}",owner = owner,repo = repo,username = username ),
  EndPoints::GetReposownerrepoCollaboratorsusernamePermission(owner,repo,username) => format!("/repos/{owner}/{repo}/collaborators/{username}/permission",owner = owner,repo = repo,username = username ),
  EndPoints::GetReposownerrepoComments(owner,repo) => format!("/repos/{owner}/{repo}/comments",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PatchReposownerrepoCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::GetReposownerrepoCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PostReposownerrepoCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoCommentscommentIdReactionsreactionId(owner,repo,comment_id,reaction_id) => format!("/repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}",owner = owner,repo = repo,comment_id = comment_id,reaction_id = reaction_id ),
  EndPoints::GetReposownerrepoCommits(owner,repo) => format!("/repos/{owner}/{repo}/commits",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoCommitscommitShaBranchesWhereHead(owner,repo,commit_sha) => format!("/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head",owner = owner,repo = repo,commit_sha = commit_sha ),
  EndPoints::GetReposownerrepoCommitscommitShaComments(owner,repo,commit_sha) => format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments",owner = owner,repo = repo,commit_sha = commit_sha ),
  EndPoints::PostReposownerrepoCommitscommitShaComments(owner,repo,commit_sha) => format!("/repos/{owner}/{repo}/commits/{commit_sha}/comments",owner = owner,repo = repo,commit_sha = commit_sha ),
  EndPoints::GetReposownerrepoCommitscommitShaPulls(owner,repo,commit_sha) => format!("/repos/{owner}/{repo}/commits/{commit_sha}/pulls",owner = owner,repo = repo,commit_sha = commit_sha ),
  EndPoints::GetReposownerrepoCommitsref(owner,repo,aref) => format!("/repos/{owner}/{repo}/commits/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoCommitsrefCheckRuns(owner,repo,aref) => format!("/repos/{owner}/{repo}/commits/{aref}/check-runs",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoCommitsrefCheckSuites(owner,repo,aref) => format!("/repos/{owner}/{repo}/commits/{aref}/check-suites",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoCommitsrefStatus(owner,repo,aref) => format!("/repos/{owner}/{repo}/commits/{aref}/status",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoCommitsrefStatuses(owner,repo,aref) => format!("/repos/{owner}/{repo}/commits/{aref}/statuses",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoCommunityProfile(owner,repo) => format!("/repos/{owner}/{repo}/community/profile",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoComparebasehead(owner,repo,basehead) => format!("/repos/{owner}/{repo}/compare/{basehead}",owner = owner,repo = repo,basehead = basehead ),
  EndPoints::GetReposownerrepoContentspath(owner,repo,path) => format!("/repos/{owner}/{repo}/contents/{path}",owner = owner,repo = repo,path = path ),
  EndPoints::PutReposownerrepoContentspath(owner,repo,path) => format!("/repos/{owner}/{repo}/contents/{path}",owner = owner,repo = repo,path = path ),
  EndPoints::DeleteReposownerrepoContentspath(owner,repo,path) => format!("/repos/{owner}/{repo}/contents/{path}",owner = owner,repo = repo,path = path ),
  EndPoints::GetReposownerrepoContributors(owner,repo) => format!("/repos/{owner}/{repo}/contributors",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoDeployments(owner,repo) => format!("/repos/{owner}/{repo}/deployments",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoDeployments(owner,repo) => format!("/repos/{owner}/{repo}/deployments",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoDeploymentsdeploymentId(owner,repo,deployment_id) => format!("/repos/{owner}/{repo}/deployments/{deployment_id}",owner = owner,repo = repo,deployment_id = deployment_id ),
  EndPoints::DeleteReposownerrepoDeploymentsdeploymentId(owner,repo,deployment_id) => format!("/repos/{owner}/{repo}/deployments/{deployment_id}",owner = owner,repo = repo,deployment_id = deployment_id ),
  EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatuses(owner,repo,deployment_id) => format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses",owner = owner,repo = repo,deployment_id = deployment_id ),
  EndPoints::PostReposownerrepoDeploymentsdeploymentIdStatuses(owner,repo,deployment_id) => format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses",owner = owner,repo = repo,deployment_id = deployment_id ),
  EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(owner,repo,deployment_id,status_id) => format!("/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}",owner = owner,repo = repo,deployment_id = deployment_id,status_id = status_id ),
  EndPoints::PostReposownerrepoDispatches(owner,repo) => format!("/repos/{owner}/{repo}/dispatches",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoEnvironments(owner,repo) => format!("/repos/{owner}/{repo}/environments",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoEnvironmentsenvironmentName(owner,repo,environment_name) => format!("/repos/{owner}/{repo}/environments/{environment_name}",owner = owner,repo = repo,environment_name = environment_name ),
  EndPoints::PutReposownerrepoEnvironmentsenvironmentName(owner,repo,environment_name) => format!("/repos/{owner}/{repo}/environments/{environment_name}",owner = owner,repo = repo,environment_name = environment_name ),
  EndPoints::DeleteReposownerrepoEnvironmentsenvironmentName(owner,repo,environment_name) => format!("/repos/{owner}/{repo}/environments/{environment_name}",owner = owner,repo = repo,environment_name = environment_name ),
  EndPoints::GetReposownerrepoEvents(owner,repo) => format!("/repos/{owner}/{repo}/events",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoForks(owner,repo) => format!("/repos/{owner}/{repo}/forks",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoForks(owner,repo) => format!("/repos/{owner}/{repo}/forks",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoGitBlobs(owner,repo) => format!("/repos/{owner}/{repo}/git/blobs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoGitBlobsfileSha(owner,repo,file_sha) => format!("/repos/{owner}/{repo}/git/blobs/{file_sha}",owner = owner,repo = repo,file_sha = file_sha ),
  EndPoints::PostReposownerrepoGitCommits(owner,repo) => format!("/repos/{owner}/{repo}/git/commits",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoGitCommitscommitSha(owner,repo,commit_sha) => format!("/repos/{owner}/{repo}/git/commits/{commit_sha}",owner = owner,repo = repo,commit_sha = commit_sha ),
  EndPoints::GetReposownerrepoGitMatchingRefsref(owner,repo,aref) => format!("/repos/{owner}/{repo}/git/matching-refs/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoGitRefref(owner,repo,aref) => format!("/repos/{owner}/{repo}/git/ref/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::PostReposownerrepoGitRefs(owner,repo) => format!("/repos/{owner}/{repo}/git/refs",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoGitRefsref(owner,repo,aref) => format!("/repos/{owner}/{repo}/git/refs/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::DeleteReposownerrepoGitRefsref(owner,repo,aref) => format!("/repos/{owner}/{repo}/git/refs/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::PostReposownerrepoGitTags(owner,repo) => format!("/repos/{owner}/{repo}/git/tags",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoGitTagstagSha(owner,repo,tag_sha) => format!("/repos/{owner}/{repo}/git/tags/{tag_sha}",owner = owner,repo = repo,tag_sha = tag_sha ),
  EndPoints::PostReposownerrepoGitTrees(owner,repo) => format!("/repos/{owner}/{repo}/git/trees",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoGitTreestreeSha(owner,repo,tree_sha) => format!("/repos/{owner}/{repo}/git/trees/{tree_sha}",owner = owner,repo = repo,tree_sha = tree_sha ),
  EndPoints::GetReposownerrepoHooks(owner,repo) => format!("/repos/{owner}/{repo}/hooks",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoHooks(owner,repo) => format!("/repos/{owner}/{repo}/hooks",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoHookshookId(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::PatchReposownerrepoHookshookId(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::DeleteReposownerrepoHookshookId(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::GetReposownerrepoHookshookIdConfig(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/config",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::PatchReposownerrepoHookshookIdConfig(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/config",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::GetReposownerrepoHookshookIdDeliveries(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::GetReposownerrepoHookshookIdDeliveriesdeliveryId(owner,repo,hook_id,delivery_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}",owner = owner,repo = repo,hook_id = hook_id,delivery_id = delivery_id ),
  EndPoints::PostReposownerrepoHookshookIdDeliveriesdeliveryIdAttempts(owner,repo,hook_id,delivery_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}/attempts",owner = owner,repo = repo,hook_id = hook_id,delivery_id = delivery_id ),
  EndPoints::PostReposownerrepoHookshookIdPings(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/pings",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::PostReposownerrepoHookshookIdTests(owner,repo,hook_id) => format!("/repos/{owner}/{repo}/hooks/{hook_id}/tests",owner = owner,repo = repo,hook_id = hook_id ),
  EndPoints::GetReposownerrepoImport(owner,repo) => format!("/repos/{owner}/{repo}/import",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoImport(owner,repo) => format!("/repos/{owner}/{repo}/import",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoImport(owner,repo) => format!("/repos/{owner}/{repo}/import",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoImport(owner,repo) => format!("/repos/{owner}/{repo}/import",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoImportAuthors(owner,repo) => format!("/repos/{owner}/{repo}/import/authors",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoImportAuthorsauthorId(owner,repo,author_id) => format!("/repos/{owner}/{repo}/import/authors/{author_id}",owner = owner,repo = repo,author_id = author_id ),
  EndPoints::GetReposownerrepoImportLargeFiles(owner,repo) => format!("/repos/{owner}/{repo}/import/large_files",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoImportLfs(owner,repo) => format!("/repos/{owner}/{repo}/import/lfs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoInstallation(owner,repo) => format!("/repos/{owner}/{repo}/installation",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoInteractionLimits(owner,repo) => format!("/repos/{owner}/{repo}/interaction-limits",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoInteractionLimits(owner,repo) => format!("/repos/{owner}/{repo}/interaction-limits",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoInteractionLimits(owner,repo) => format!("/repos/{owner}/{repo}/interaction-limits",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoInvitations(owner,repo) => format!("/repos/{owner}/{repo}/invitations",owner = owner,repo = repo ),
  EndPoints::PatchReposownerrepoInvitationsinvitationId(owner,repo,invitation_id) => format!("/repos/{owner}/{repo}/invitations/{invitation_id}",owner = owner,repo = repo,invitation_id = invitation_id ),
  EndPoints::DeleteReposownerrepoInvitationsinvitationId(owner,repo,invitation_id) => format!("/repos/{owner}/{repo}/invitations/{invitation_id}",owner = owner,repo = repo,invitation_id = invitation_id ),
  EndPoints::GetReposownerrepoIssues(owner,repo) => format!("/repos/{owner}/{repo}/issues",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoIssues(owner,repo) => format!("/repos/{owner}/{repo}/issues",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoIssuesComments(owner,repo) => format!("/repos/{owner}/{repo}/issues/comments",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoIssuesCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PatchReposownerrepoIssuesCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoIssuesCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::GetReposownerrepoIssuesCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PostReposownerrepoIssuesCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoIssuesCommentscommentIdReactionsreactionId(owner,repo,comment_id,reaction_id) => format!("/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}",owner = owner,repo = repo,comment_id = comment_id,reaction_id = reaction_id ),
  EndPoints::GetReposownerrepoIssuesEvents(owner,repo) => format!("/repos/{owner}/{repo}/issues/events",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoIssuesEventseventId(owner,repo,event_id) => format!("/repos/{owner}/{repo}/issues/events/{event_id}",owner = owner,repo = repo,event_id = event_id ),
  EndPoints::GetReposownerrepoIssuesissueNumber(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PatchReposownerrepoIssuesissueNumber(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PostReposownerrepoIssuesissueNumberAssignees(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::DeleteReposownerrepoIssuesissueNumberAssignees(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/assignees",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::GetReposownerrepoIssuesissueNumberComments(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/comments",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PostReposownerrepoIssuesissueNumberComments(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/comments",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::GetReposownerrepoIssuesissueNumberEvents(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/events",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::GetReposownerrepoIssuesissueNumberLabels(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/labels",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PostReposownerrepoIssuesissueNumberLabels(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/labels",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PutReposownerrepoIssuesissueNumberLabels(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/labels",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::DeleteReposownerrepoIssuesissueNumberLabels(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/labels",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::DeleteReposownerrepoIssuesissueNumberLabelsname(owner,repo,issue_number,name) => format!("/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}",owner = owner,repo = repo,issue_number = issue_number,name = name ),
  EndPoints::PutReposownerrepoIssuesissueNumberLock(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/lock",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::DeleteReposownerrepoIssuesissueNumberLock(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/lock",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::GetReposownerrepoIssuesissueNumberReactions(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::PostReposownerrepoIssuesissueNumberReactions(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::DeleteReposownerrepoIssuesissueNumberReactionsreactionId(owner,repo,issue_number,reaction_id) => format!("/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}",owner = owner,repo = repo,issue_number = issue_number,reaction_id = reaction_id ),
  EndPoints::GetReposownerrepoIssuesissueNumberTimeline(owner,repo,issue_number) => format!("/repos/{owner}/{repo}/issues/{issue_number}/timeline",owner = owner,repo = repo,issue_number = issue_number ),
  EndPoints::GetReposownerrepoKeys(owner,repo) => format!("/repos/{owner}/{repo}/keys",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoKeys(owner,repo) => format!("/repos/{owner}/{repo}/keys",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoKeyskeyId(owner,repo,key_id) => format!("/repos/{owner}/{repo}/keys/{key_id}",owner = owner,repo = repo,key_id = key_id ),
  EndPoints::DeleteReposownerrepoKeyskeyId(owner,repo,key_id) => format!("/repos/{owner}/{repo}/keys/{key_id}",owner = owner,repo = repo,key_id = key_id ),
  EndPoints::GetReposownerrepoLabels(owner,repo) => format!("/repos/{owner}/{repo}/labels",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoLabels(owner,repo) => format!("/repos/{owner}/{repo}/labels",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoLabelsname(owner,repo,name) => format!("/repos/{owner}/{repo}/labels/{name}",owner = owner,repo = repo,name = name ),
  EndPoints::PatchReposownerrepoLabelsname(owner,repo,name) => format!("/repos/{owner}/{repo}/labels/{name}",owner = owner,repo = repo,name = name ),
  EndPoints::DeleteReposownerrepoLabelsname(owner,repo,name) => format!("/repos/{owner}/{repo}/labels/{name}",owner = owner,repo = repo,name = name ),
  EndPoints::GetReposownerrepoLanguages(owner,repo) => format!("/repos/{owner}/{repo}/languages",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoLfs(owner,repo) => format!("/repos/{owner}/{repo}/lfs",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoLfs(owner,repo) => format!("/repos/{owner}/{repo}/lfs",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoLicense(owner,repo) => format!("/repos/{owner}/{repo}/license",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoMergeUpstream(owner,repo) => format!("/repos/{owner}/{repo}/merge-upstream",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoMerges(owner,repo) => format!("/repos/{owner}/{repo}/merges",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoMilestones(owner,repo) => format!("/repos/{owner}/{repo}/milestones",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoMilestones(owner,repo) => format!("/repos/{owner}/{repo}/milestones",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoMilestonesmilestoneNumber(owner,repo,milestone_number) => format!("/repos/{owner}/{repo}/milestones/{milestone_number}",owner = owner,repo = repo,milestone_number = milestone_number ),
  EndPoints::PatchReposownerrepoMilestonesmilestoneNumber(owner,repo,milestone_number) => format!("/repos/{owner}/{repo}/milestones/{milestone_number}",owner = owner,repo = repo,milestone_number = milestone_number ),
  EndPoints::DeleteReposownerrepoMilestonesmilestoneNumber(owner,repo,milestone_number) => format!("/repos/{owner}/{repo}/milestones/{milestone_number}",owner = owner,repo = repo,milestone_number = milestone_number ),
  EndPoints::GetReposownerrepoMilestonesmilestoneNumberLabels(owner,repo,milestone_number) => format!("/repos/{owner}/{repo}/milestones/{milestone_number}/labels",owner = owner,repo = repo,milestone_number = milestone_number ),
  EndPoints::GetReposownerrepoNotifications(owner,repo) => format!("/repos/{owner}/{repo}/notifications",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoNotifications(owner,repo) => format!("/repos/{owner}/{repo}/notifications",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPages(owner,repo) => format!("/repos/{owner}/{repo}/pages",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoPages(owner,repo) => format!("/repos/{owner}/{repo}/pages",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoPages(owner,repo) => format!("/repos/{owner}/{repo}/pages",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoPages(owner,repo) => format!("/repos/{owner}/{repo}/pages",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPagesBuilds(owner,repo) => format!("/repos/{owner}/{repo}/pages/builds",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoPagesBuilds(owner,repo) => format!("/repos/{owner}/{repo}/pages/builds",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPagesBuildsLatest(owner,repo) => format!("/repos/{owner}/{repo}/pages/builds/latest",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPagesBuildsbuildId(owner,repo,build_id) => format!("/repos/{owner}/{repo}/pages/builds/{build_id}",owner = owner,repo = repo,build_id = build_id ),
  EndPoints::GetReposownerrepoPagesHealth(owner,repo) => format!("/repos/{owner}/{repo}/pages/health",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoProjects(owner,repo) => format!("/repos/{owner}/{repo}/projects",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoProjects(owner,repo) => format!("/repos/{owner}/{repo}/projects",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPulls(owner,repo) => format!("/repos/{owner}/{repo}/pulls",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoPulls(owner,repo) => format!("/repos/{owner}/{repo}/pulls",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPullsComments(owner,repo) => format!("/repos/{owner}/{repo}/pulls/comments",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoPullsCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PatchReposownerrepoPullsCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoPullsCommentscommentId(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::GetReposownerrepoPullsCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::PostReposownerrepoPullsCommentscommentIdReactions(owner,repo,comment_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions",owner = owner,repo = repo,comment_id = comment_id ),
  EndPoints::DeleteReposownerrepoPullsCommentscommentIdReactionsreactionId(owner,repo,comment_id,reaction_id) => format!("/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}",owner = owner,repo = repo,comment_id = comment_id,reaction_id = reaction_id ),
  EndPoints::GetReposownerrepoPullspullNumber(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PatchReposownerrepoPullspullNumber(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PostReposownerrepoPullspullNumberCodespaces(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/codespaces",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberComments(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PostReposownerrepoPullspullNumberComments(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PostReposownerrepoPullspullNumberCommentscommentIdReplies(owner,repo,pull_number,comment_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies",owner = owner,repo = repo,pull_number = pull_number,comment_id = comment_id ),
  EndPoints::GetReposownerrepoPullspullNumberCommits(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/commits",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberFiles(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/files",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberMerge(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/merge",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PutReposownerrepoPullspullNumberMerge(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/merge",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberRequestedReviewers(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PostReposownerrepoPullspullNumberRequestedReviewers(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::DeleteReposownerrepoPullspullNumberRequestedReviewers(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberReviews(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::PostReposownerrepoPullspullNumberReviews(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoPullspullNumberReviewsreviewId(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::PutReposownerrepoPullspullNumberReviewsreviewId(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::DeleteReposownerrepoPullspullNumberReviewsreviewId(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::GetReposownerrepoPullspullNumberReviewsreviewIdComments(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::PostReposownerrepoPullspullNumberReviewsreviewIdEvents(owner,repo,pull_number,review_id) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events",owner = owner,repo = repo,pull_number = pull_number,review_id = review_id ),
  EndPoints::PutReposownerrepoPullspullNumberUpdateBranch(owner,repo,pull_number) => format!("/repos/{owner}/{repo}/pulls/{pull_number}/update-branch",owner = owner,repo = repo,pull_number = pull_number ),
  EndPoints::GetReposownerrepoReadme(owner,repo) => format!("/repos/{owner}/{repo}/readme",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoReadmedir(owner,repo,dir) => format!("/repos/{owner}/{repo}/readme/{dir}",owner = owner,repo = repo,dir = dir ),
  EndPoints::GetReposownerrepoReleases(owner,repo) => format!("/repos/{owner}/{repo}/releases",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoReleases(owner,repo) => format!("/repos/{owner}/{repo}/releases",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoReleasesAssetsassetId(owner,repo,asset_id) => format!("/repos/{owner}/{repo}/releases/assets/{asset_id}",owner = owner,repo = repo,asset_id = asset_id ),
  EndPoints::PatchReposownerrepoReleasesAssetsassetId(owner,repo,asset_id) => format!("/repos/{owner}/{repo}/releases/assets/{asset_id}",owner = owner,repo = repo,asset_id = asset_id ),
  EndPoints::DeleteReposownerrepoReleasesAssetsassetId(owner,repo,asset_id) => format!("/repos/{owner}/{repo}/releases/assets/{asset_id}",owner = owner,repo = repo,asset_id = asset_id ),
  EndPoints::PostReposownerrepoReleasesGenerateNotes(owner,repo) => format!("/repos/{owner}/{repo}/releases/generate-notes",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoReleasesLatest(owner,repo) => format!("/repos/{owner}/{repo}/releases/latest",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoReleasesTagstag(owner,repo,tag) => format!("/repos/{owner}/{repo}/releases/tags/{tag}",owner = owner,repo = repo,tag = tag ),
  EndPoints::GetReposownerrepoReleasesreleaseId(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::PatchReposownerrepoReleasesreleaseId(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::DeleteReposownerrepoReleasesreleaseId(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::GetReposownerrepoReleasesreleaseIdAssets(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}/assets",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::PostReposownerrepoReleasesreleaseIdAssets(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}/assets",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::PostReposownerrepoReleasesreleaseIdReactions(owner,repo,release_id) => format!("/repos/{owner}/{repo}/releases/{release_id}/reactions",owner = owner,repo = repo,release_id = release_id ),
  EndPoints::GetReposownerrepoSecretScanningAlerts(owner,repo) => format!("/repos/{owner}/{repo}/secret-scanning/alerts",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoSecretScanningAlertsalertNumber(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::PatchReposownerrepoSecretScanningAlertsalertNumber(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::GetReposownerrepoSecretScanningAlertsalertNumberLocations(owner,repo,alert_number) => format!("/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations",owner = owner,repo = repo,alert_number = alert_number ),
  EndPoints::GetReposownerrepoStargazers(owner,repo) => format!("/repos/{owner}/{repo}/stargazers",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoStatsCodeFrequency(owner,repo) => format!("/repos/{owner}/{repo}/stats/code_frequency",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoStatsCommitActivity(owner,repo) => format!("/repos/{owner}/{repo}/stats/commit_activity",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoStatsContributors(owner,repo) => format!("/repos/{owner}/{repo}/stats/contributors",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoStatsParticipation(owner,repo) => format!("/repos/{owner}/{repo}/stats/participation",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoStatsPunchCard(owner,repo) => format!("/repos/{owner}/{repo}/stats/punch_card",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoStatusessha(owner,repo,sha) => format!("/repos/{owner}/{repo}/statuses/{sha}",owner = owner,repo = repo,sha = sha ),
  EndPoints::GetReposownerrepoSubscribers(owner,repo) => format!("/repos/{owner}/{repo}/subscribers",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoSubscription(owner,repo) => format!("/repos/{owner}/{repo}/subscription",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoSubscription(owner,repo) => format!("/repos/{owner}/{repo}/subscription",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoSubscription(owner,repo) => format!("/repos/{owner}/{repo}/subscription",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTags(owner,repo) => format!("/repos/{owner}/{repo}/tags",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTarballref(owner,repo,aref) => format!("/repos/{owner}/{repo}/tarball/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::GetReposownerrepoTeams(owner,repo) => format!("/repos/{owner}/{repo}/teams",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTopics(owner,repo) => format!("/repos/{owner}/{repo}/topics",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoTopics(owner,repo) => format!("/repos/{owner}/{repo}/topics",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTrafficClones(owner,repo) => format!("/repos/{owner}/{repo}/traffic/clones",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTrafficPopularPaths(owner,repo) => format!("/repos/{owner}/{repo}/traffic/popular/paths",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTrafficPopularReferrers(owner,repo) => format!("/repos/{owner}/{repo}/traffic/popular/referrers",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoTrafficViews(owner,repo) => format!("/repos/{owner}/{repo}/traffic/views",owner = owner,repo = repo ),
  EndPoints::PostReposownerrepoTransfer(owner,repo) => format!("/repos/{owner}/{repo}/transfer",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoVulnerabilityAlerts(owner,repo) => format!("/repos/{owner}/{repo}/vulnerability-alerts",owner = owner,repo = repo ),
  EndPoints::PutReposownerrepoVulnerabilityAlerts(owner,repo) => format!("/repos/{owner}/{repo}/vulnerability-alerts",owner = owner,repo = repo ),
  EndPoints::DeleteReposownerrepoVulnerabilityAlerts(owner,repo) => format!("/repos/{owner}/{repo}/vulnerability-alerts",owner = owner,repo = repo ),
  EndPoints::GetReposownerrepoZipballref(owner,repo,aref) => format!("/repos/{owner}/{repo}/zipball/{aref}",owner = owner,repo = repo,aref = aref ),
  EndPoints::PostRepostemplateOwnertemplateRepoGenerate(template_owner,template_repo) => format!("/repos/{template_owner}/{template_repo}/generate",template_owner = template_owner,template_repo = template_repo ),
  EndPoints::GetRepositories() => format!("/repositories", ),
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(repository_id,environment_name) => format!("/repositories/{repository_id}/environments/{environment_name}/secrets",repository_id = repository_id,environment_name = environment_name ),
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(repository_id,environment_name) => format!("/repositories/{repository_id}/environments/{environment_name}/secrets/public-key",repository_id = repository_id,environment_name = environment_name ),
  EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(repository_id,environment_name,secret_name) => format!("/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}",repository_id = repository_id,environment_name = environment_name,secret_name = secret_name ),
  EndPoints::PutRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(repository_id,environment_name,secret_name) => format!("/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}",repository_id = repository_id,environment_name = environment_name,secret_name = secret_name ),
  EndPoints::DeleteRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(repository_id,environment_name,secret_name) => format!("/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}",repository_id = repository_id,environment_name = environment_name,secret_name = secret_name ),
  EndPoints::GetScimV2EnterprisesenterpriseGroups(enterprise) => format!("/scim/v2/enterprises/{enterprise}/Groups",enterprise = enterprise ),
  EndPoints::PostScimV2EnterprisesenterpriseGroups(enterprise) => format!("/scim/v2/enterprises/{enterprise}/Groups",enterprise = enterprise ),
  EndPoints::GetScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise,scim_group_id) => format!("/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}",enterprise = enterprise,scim_group_id = scim_group_id ),
  EndPoints::PutScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise,scim_group_id) => format!("/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}",enterprise = enterprise,scim_group_id = scim_group_id ),
  EndPoints::PatchScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise,scim_group_id) => format!("/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}",enterprise = enterprise,scim_group_id = scim_group_id ),
  EndPoints::DeleteScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise,scim_group_id) => format!("/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}",enterprise = enterprise,scim_group_id = scim_group_id ),
  EndPoints::GetScimV2EnterprisesenterpriseUsers(enterprise) => format!("/scim/v2/enterprises/{enterprise}/Users",enterprise = enterprise ),
  EndPoints::PostScimV2EnterprisesenterpriseUsers(enterprise) => format!("/scim/v2/enterprises/{enterprise}/Users",enterprise = enterprise ),
  EndPoints::GetScimV2EnterprisesenterpriseUsersscimUserId(enterprise,scim_user_id) => format!("/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}",enterprise = enterprise,scim_user_id = scim_user_id ),
  EndPoints::PutScimV2EnterprisesenterpriseUsersscimUserId(enterprise,scim_user_id) => format!("/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}",enterprise = enterprise,scim_user_id = scim_user_id ),
  EndPoints::PatchScimV2EnterprisesenterpriseUsersscimUserId(enterprise,scim_user_id) => format!("/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}",enterprise = enterprise,scim_user_id = scim_user_id ),
  EndPoints::DeleteScimV2EnterprisesenterpriseUsersscimUserId(enterprise,scim_user_id) => format!("/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}",enterprise = enterprise,scim_user_id = scim_user_id ),
  EndPoints::GetScimV2OrganizationsorgUsers(org) => format!("/scim/v2/organizations/{org}/Users",org = org ),
  EndPoints::PostScimV2OrganizationsorgUsers(org) => format!("/scim/v2/organizations/{org}/Users",org = org ),
  EndPoints::GetScimV2OrganizationsorgUsersscimUserId(org,scim_user_id) => format!("/scim/v2/organizations/{org}/Users/{scim_user_id}",org = org,scim_user_id = scim_user_id ),
  EndPoints::PutScimV2OrganizationsorgUsersscimUserId(org,scim_user_id) => format!("/scim/v2/organizations/{org}/Users/{scim_user_id}",org = org,scim_user_id = scim_user_id ),
  EndPoints::PatchScimV2OrganizationsorgUsersscimUserId(org,scim_user_id) => format!("/scim/v2/organizations/{org}/Users/{scim_user_id}",org = org,scim_user_id = scim_user_id ),
  EndPoints::DeleteScimV2OrganizationsorgUsersscimUserId(org,scim_user_id) => format!("/scim/v2/organizations/{org}/Users/{scim_user_id}",org = org,scim_user_id = scim_user_id ),
  EndPoints::GetSearchCode() => format!("/search/code", ),
  EndPoints::GetSearchCommits() => format!("/search/commits", ),
  EndPoints::GetSearchIssues() => format!("/search/issues", ),
  EndPoints::GetSearchLabels() => format!("/search/labels", ),
  EndPoints::GetSearchRepositories() => format!("/search/repositories", ),
  EndPoints::GetSearchTopics() => format!("/search/topics", ),
  EndPoints::GetSearchUsers() => format!("/search/users", ),
  EndPoints::GetTeamsteamId(team_id) => format!("/teams/{team_id}",team_id = team_id ),
  EndPoints::PatchTeamsteamId(team_id) => format!("/teams/{team_id}",team_id = team_id ),
  EndPoints::DeleteTeamsteamId(team_id) => format!("/teams/{team_id}",team_id = team_id ),
  EndPoints::GetTeamsteamIdDiscussions(team_id) => format!("/teams/{team_id}/discussions",team_id = team_id ),
  EndPoints::PostTeamsteamIdDiscussions(team_id) => format!("/teams/{team_id}/discussions",team_id = team_id ),
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumber(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumber(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::DeleteTeamsteamIdDiscussionsdiscussionNumber(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberComments(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberComments(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(team_id,discussion_number,comment_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}",team_id = team_id,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(team_id,discussion_number,comment_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}",team_id = team_id,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::DeleteTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(team_id,discussion_number,comment_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}",team_id = team_id,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(team_id,discussion_number,comment_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions",team_id = team_id,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberReactions(team_id,discussion_number,comment_number) => format!("/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions",team_id = team_id,discussion_number = discussion_number,comment_number = comment_number ),
  EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberReactions(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}/reactions",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::PostTeamsteamIdDiscussionsdiscussionNumberReactions(team_id,discussion_number) => format!("/teams/{team_id}/discussions/{discussion_number}/reactions",team_id = team_id,discussion_number = discussion_number ),
  EndPoints::GetTeamsteamIdInvitations(team_id) => format!("/teams/{team_id}/invitations",team_id = team_id ),
  EndPoints::GetTeamsteamIdMembers(team_id) => format!("/teams/{team_id}/members",team_id = team_id ),
  EndPoints::GetTeamsteamIdMembersusername(team_id,username) => format!("/teams/{team_id}/members/{username}",team_id = team_id,username = username ),
  EndPoints::PutTeamsteamIdMembersusername(team_id,username) => format!("/teams/{team_id}/members/{username}",team_id = team_id,username = username ),
  EndPoints::DeleteTeamsteamIdMembersusername(team_id,username) => format!("/teams/{team_id}/members/{username}",team_id = team_id,username = username ),
  EndPoints::GetTeamsteamIdMembershipsusername(team_id,username) => format!("/teams/{team_id}/memberships/{username}",team_id = team_id,username = username ),
  EndPoints::PutTeamsteamIdMembershipsusername(team_id,username) => format!("/teams/{team_id}/memberships/{username}",team_id = team_id,username = username ),
  EndPoints::DeleteTeamsteamIdMembershipsusername(team_id,username) => format!("/teams/{team_id}/memberships/{username}",team_id = team_id,username = username ),
  EndPoints::GetTeamsteamIdProjects(team_id) => format!("/teams/{team_id}/projects",team_id = team_id ),
  EndPoints::GetTeamsteamIdProjectsprojectId(team_id,project_id) => format!("/teams/{team_id}/projects/{project_id}",team_id = team_id,project_id = project_id ),
  EndPoints::PutTeamsteamIdProjectsprojectId(team_id,project_id) => format!("/teams/{team_id}/projects/{project_id}",team_id = team_id,project_id = project_id ),
  EndPoints::DeleteTeamsteamIdProjectsprojectId(team_id,project_id) => format!("/teams/{team_id}/projects/{project_id}",team_id = team_id,project_id = project_id ),
  EndPoints::GetTeamsteamIdRepos(team_id) => format!("/teams/{team_id}/repos",team_id = team_id ),
  EndPoints::GetTeamsteamIdReposownerrepo(team_id,owner,repo) => format!("/teams/{team_id}/repos/{owner}/{repo}",team_id = team_id,owner = owner,repo = repo ),
  EndPoints::PutTeamsteamIdReposownerrepo(team_id,owner,repo) => format!("/teams/{team_id}/repos/{owner}/{repo}",team_id = team_id,owner = owner,repo = repo ),
  EndPoints::DeleteTeamsteamIdReposownerrepo(team_id,owner,repo) => format!("/teams/{team_id}/repos/{owner}/{repo}",team_id = team_id,owner = owner,repo = repo ),
  EndPoints::GetTeamsteamIdTeamSyncGroupMappings(team_id) => format!("/teams/{team_id}/team-sync/group-mappings",team_id = team_id ),
  EndPoints::PatchTeamsteamIdTeamSyncGroupMappings(team_id) => format!("/teams/{team_id}/team-sync/group-mappings",team_id = team_id ),
  EndPoints::GetTeamsteamIdTeams(team_id) => format!("/teams/{team_id}/teams",team_id = team_id ),
  EndPoints::GetUser() => format!("/user", ),
  EndPoints::PatchUser() => format!("/user", ),
  EndPoints::GetUserBlocks() => format!("/user/blocks", ),
  EndPoints::GetUserBlocksusername(username) => format!("/user/blocks/{username}",username = username ),
  EndPoints::PutUserBlocksusername(username) => format!("/user/blocks/{username}",username = username ),
  EndPoints::DeleteUserBlocksusername(username) => format!("/user/blocks/{username}",username = username ),
  EndPoints::GetUserCodespaces() => format!("/user/codespaces", ),
  EndPoints::PostUserCodespaces() => format!("/user/codespaces", ),
  EndPoints::GetUserCodespacesSecrets() => format!("/user/codespaces/secrets", ),
  EndPoints::GetUserCodespacesSecretsPublicKey() => format!("/user/codespaces/secrets/public-key", ),
  EndPoints::GetUserCodespacesSecretssecretName(secret_name) => format!("/user/codespaces/secrets/{secret_name}",secret_name = secret_name ),
  EndPoints::PutUserCodespacesSecretssecretName(secret_name) => format!("/user/codespaces/secrets/{secret_name}",secret_name = secret_name ),
  EndPoints::DeleteUserCodespacesSecretssecretName(secret_name) => format!("/user/codespaces/secrets/{secret_name}",secret_name = secret_name ),
  EndPoints::GetUserCodespacesSecretssecretNameRepositories(secret_name) => format!("/user/codespaces/secrets/{secret_name}/repositories",secret_name = secret_name ),
  EndPoints::PutUserCodespacesSecretssecretNameRepositories(secret_name) => format!("/user/codespaces/secrets/{secret_name}/repositories",secret_name = secret_name ),
  EndPoints::PutUserCodespacesSecretssecretNameRepositoriesrepositoryId(secret_name,repository_id) => format!("/user/codespaces/secrets/{secret_name}/repositories/{repository_id}",secret_name = secret_name,repository_id = repository_id ),
  EndPoints::DeleteUserCodespacesSecretssecretNameRepositoriesrepositoryId(secret_name,repository_id) => format!("/user/codespaces/secrets/{secret_name}/repositories/{repository_id}",secret_name = secret_name,repository_id = repository_id ),
  EndPoints::GetUserCodespacescodespaceName(codespace_name) => format!("/user/codespaces/{codespace_name}",codespace_name = codespace_name ),
  EndPoints::PatchUserCodespacescodespaceName(codespace_name) => format!("/user/codespaces/{codespace_name}",codespace_name = codespace_name ),
  EndPoints::DeleteUserCodespacescodespaceName(codespace_name) => format!("/user/codespaces/{codespace_name}",codespace_name = codespace_name ),
  EndPoints::GetUserCodespacescodespaceNameMachines(codespace_name) => format!("/user/codespaces/{codespace_name}/machines",codespace_name = codespace_name ),
  EndPoints::PostUserCodespacescodespaceNameStart(codespace_name) => format!("/user/codespaces/{codespace_name}/start",codespace_name = codespace_name ),
  EndPoints::PostUserCodespacescodespaceNameStop(codespace_name) => format!("/user/codespaces/{codespace_name}/stop",codespace_name = codespace_name ),
  EndPoints::PatchUserEmailVisibility() => format!("/user/email/visibility", ),
  EndPoints::GetUserEmails() => format!("/user/emails", ),
  EndPoints::PostUserEmails() => format!("/user/emails", ),
  EndPoints::DeleteUserEmails() => format!("/user/emails", ),
  EndPoints::GetUserFollowers() => format!("/user/followers", ),
  EndPoints::GetUserFollowing() => format!("/user/following", ),
  EndPoints::GetUserFollowingusername(username) => format!("/user/following/{username}",username = username ),
  EndPoints::PutUserFollowingusername(username) => format!("/user/following/{username}",username = username ),
  EndPoints::DeleteUserFollowingusername(username) => format!("/user/following/{username}",username = username ),
  EndPoints::GetUserGpgKeys() => format!("/user/gpg_keys", ),
  EndPoints::PostUserGpgKeys() => format!("/user/gpg_keys", ),
  EndPoints::GetUserGpgKeysgpgKeyId(gpg_key_id) => format!("/user/gpg_keys/{gpg_key_id}",gpg_key_id = gpg_key_id ),
  EndPoints::DeleteUserGpgKeysgpgKeyId(gpg_key_id) => format!("/user/gpg_keys/{gpg_key_id}",gpg_key_id = gpg_key_id ),
  EndPoints::GetUserInstallations() => format!("/user/installations", ),
  EndPoints::GetUserInstallationsinstallationIdRepositories(installation_id) => format!("/user/installations/{installation_id}/repositories",installation_id = installation_id ),
  EndPoints::PutUserInstallationsinstallationIdRepositoriesrepositoryId(installation_id,repository_id) => format!("/user/installations/{installation_id}/repositories/{repository_id}",installation_id = installation_id,repository_id = repository_id ),
  EndPoints::DeleteUserInstallationsinstallationIdRepositoriesrepositoryId(installation_id,repository_id) => format!("/user/installations/{installation_id}/repositories/{repository_id}",installation_id = installation_id,repository_id = repository_id ),
  EndPoints::GetUserInteractionLimits() => format!("/user/interaction-limits", ),
  EndPoints::PutUserInteractionLimits() => format!("/user/interaction-limits", ),
  EndPoints::DeleteUserInteractionLimits() => format!("/user/interaction-limits", ),
  EndPoints::GetUserIssues() => format!("/user/issues", ),
  EndPoints::GetUserKeys() => format!("/user/keys", ),
  EndPoints::PostUserKeys() => format!("/user/keys", ),
  EndPoints::GetUserKeyskeyId(key_id) => format!("/user/keys/{key_id}",key_id = key_id ),
  EndPoints::DeleteUserKeyskeyId(key_id) => format!("/user/keys/{key_id}",key_id = key_id ),
  EndPoints::GetUserMarketplacePurchases() => format!("/user/marketplace_purchases", ),
  EndPoints::GetUserMarketplacePurchasesStubbed() => format!("/user/marketplace_purchases/stubbed", ),
  EndPoints::GetUserMembershipsOrgs() => format!("/user/memberships/orgs", ),
  EndPoints::GetUserMembershipsOrgsorg(org) => format!("/user/memberships/orgs/{org}",org = org ),
  EndPoints::PatchUserMembershipsOrgsorg(org) => format!("/user/memberships/orgs/{org}",org = org ),
  EndPoints::GetUserMigrations() => format!("/user/migrations", ),
  EndPoints::PostUserMigrations() => format!("/user/migrations", ),
  EndPoints::GetUserMigrationsmigrationId(migration_id) => format!("/user/migrations/{migration_id}",migration_id = migration_id ),
  EndPoints::GetUserMigrationsmigrationIdArchive(migration_id) => format!("/user/migrations/{migration_id}/archive",migration_id = migration_id ),
  EndPoints::DeleteUserMigrationsmigrationIdArchive(migration_id) => format!("/user/migrations/{migration_id}/archive",migration_id = migration_id ),
  EndPoints::DeleteUserMigrationsmigrationIdReposrepoNameLock(migration_id,repo_name) => format!("/user/migrations/{migration_id}/repos/{repo_name}/lock",migration_id = migration_id,repo_name = repo_name ),
  EndPoints::GetUserMigrationsmigrationIdRepositories(migration_id) => format!("/user/migrations/{migration_id}/repositories",migration_id = migration_id ),
  EndPoints::GetUserOrgs() => format!("/user/orgs", ),
  EndPoints::GetUserPackages() => format!("/user/packages", ),
  EndPoints::GetUserPackagespackageTypepackageName(package_type,package_name) => format!("/user/packages/{package_type}/{package_name}",package_type = package_type,package_name = package_name ),
  EndPoints::DeleteUserPackagespackageTypepackageName(package_type,package_name) => format!("/user/packages/{package_type}/{package_name}",package_type = package_type,package_name = package_name ),
  EndPoints::PostUserPackagespackageTypepackageNameRestore(package_type,package_name) => format!("/user/packages/{package_type}/{package_name}/restore",package_type = package_type,package_name = package_name ),
  EndPoints::GetUserPackagespackageTypepackageNameVersions(package_type,package_name) => format!("/user/packages/{package_type}/{package_name}/versions",package_type = package_type,package_name = package_name ),
  EndPoints::GetUserPackagespackageTypepackageNameVersionspackageVersionId(package_type,package_name,package_version_id) => format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}",package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::DeleteUserPackagespackageTypepackageNameVersionspackageVersionId(package_type,package_name,package_version_id) => format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}",package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::PostUserPackagespackageTypepackageNameVersionspackageVersionIdRestore(package_type,package_name,package_version_id) => format!("/user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore",package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::PostUserProjects() => format!("/user/projects", ),
  EndPoints::GetUserPublicEmails() => format!("/user/public_emails", ),
  EndPoints::GetUserRepos() => format!("/user/repos", ),
  EndPoints::PostUserRepos() => format!("/user/repos", ),
  EndPoints::GetUserRepositoryInvitations() => format!("/user/repository_invitations", ),
  EndPoints::PatchUserRepositoryInvitationsinvitationId(invitation_id) => format!("/user/repository_invitations/{invitation_id}",invitation_id = invitation_id ),
  EndPoints::DeleteUserRepositoryInvitationsinvitationId(invitation_id) => format!("/user/repository_invitations/{invitation_id}",invitation_id = invitation_id ),
  EndPoints::GetUserStarred() => format!("/user/starred", ),
  EndPoints::GetUserStarredownerrepo(owner,repo) => format!("/user/starred/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::PutUserStarredownerrepo(owner,repo) => format!("/user/starred/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::DeleteUserStarredownerrepo(owner,repo) => format!("/user/starred/{owner}/{repo}",owner = owner,repo = repo ),
  EndPoints::GetUserSubscriptions() => format!("/user/subscriptions", ),
  EndPoints::GetUserTeams() => format!("/user/teams", ),
  EndPoints::GetUsers() => format!("/users", ),
  EndPoints::GetUsersusername(username) => format!("/users/{username}",username = username ),
  EndPoints::GetUsersusernameEvents(username) => format!("/users/{username}/events",username = username ),
  EndPoints::GetUsersusernameEventsOrgsorg(username,org) => format!("/users/{username}/events/orgs/{org}",username = username,org = org ),
  EndPoints::GetUsersusernameEventsPublic(username) => format!("/users/{username}/events/public",username = username ),
  EndPoints::GetUsersusernameFollowers(username) => format!("/users/{username}/followers",username = username ),
  EndPoints::GetUsersusernameFollowing(username) => format!("/users/{username}/following",username = username ),
  EndPoints::GetUsersusernameFollowingtargetUser(username,target_user) => format!("/users/{username}/following/{target_user}",username = username,target_user = target_user ),
  EndPoints::GetUsersusernameGists(username) => format!("/users/{username}/gists",username = username ),
  EndPoints::GetUsersusernameGpgKeys(username) => format!("/users/{username}/gpg_keys",username = username ),
  EndPoints::GetUsersusernameHovercard(username) => format!("/users/{username}/hovercard",username = username ),
  EndPoints::GetUsersusernameInstallation(username) => format!("/users/{username}/installation",username = username ),
  EndPoints::GetUsersusernameKeys(username) => format!("/users/{username}/keys",username = username ),
  EndPoints::GetUsersusernameOrgs(username) => format!("/users/{username}/orgs",username = username ),
  EndPoints::GetUsersusernamePackages(username) => format!("/users/{username}/packages",username = username ),
  EndPoints::GetUsersusernamePackagespackageTypepackageName(username,package_type,package_name) => format!("/users/{username}/packages/{package_type}/{package_name}",username = username,package_type = package_type,package_name = package_name ),
  EndPoints::DeleteUsersusernamePackagespackageTypepackageName(username,package_type,package_name) => format!("/users/{username}/packages/{package_type}/{package_name}",username = username,package_type = package_type,package_name = package_name ),
  EndPoints::PostUsersusernamePackagespackageTypepackageNameRestore(username,package_type,package_name) => format!("/users/{username}/packages/{package_type}/{package_name}/restore",username = username,package_type = package_type,package_name = package_name ),
  EndPoints::GetUsersusernamePackagespackageTypepackageNameVersions(username,package_type,package_name) => format!("/users/{username}/packages/{package_type}/{package_name}/versions",username = username,package_type = package_type,package_name = package_name ),
  EndPoints::GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(username,package_type,package_name,package_version_id) => format!("/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}",username = username,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::DeleteUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(username,package_type,package_name,package_version_id) => format!("/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}",username = username,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::PostUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdRestore(username,package_type,package_name,package_version_id) => format!("/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore",username = username,package_type = package_type,package_name = package_name,package_version_id = package_version_id ),
  EndPoints::GetUsersusernameProjects(username) => format!("/users/{username}/projects",username = username ),
  EndPoints::GetUsersusernameReceivedEvents(username) => format!("/users/{username}/received_events",username = username ),
  EndPoints::GetUsersusernameReceivedEventsPublic(username) => format!("/users/{username}/received_events/public",username = username ),
  EndPoints::GetUsersusernameRepos(username) => format!("/users/{username}/repos",username = username ),
  EndPoints::GetUsersusernameSettingsBillingActions(username) => format!("/users/{username}/settings/billing/actions",username = username ),
  EndPoints::GetUsersusernameSettingsBillingPackages(username) => format!("/users/{username}/settings/billing/packages",username = username ),
  EndPoints::GetUsersusernameSettingsBillingSharedStorage(username) => format!("/users/{username}/settings/billing/shared-storage",username = username ),
  EndPoints::GetUsersusernameStarred(username) => format!("/users/{username}/starred",username = username ),
  EndPoints::GetUsersusernameSubscriptions(username) => format!("/users/{username}/subscriptions",username = username ),
  EndPoints::GetZen() => format!("/zen", )}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetResponse {
    ///
    current_user_url: String,
    ///
    current_user_authorizations_html_url: String,
    ///
    authorizations_url: String,
    ///
    code_search_url: String,
    ///
    commit_search_url: String,
    ///
    emails_url: String,
    ///
    emojis_url: String,
    ///
    events_url: String,
    ///
    feeds_url: String,
    ///
    followers_url: String,
    ///
    following_url: String,
    ///
    gists_url: String,
    ///
    hub_url: String,
    ///
    issue_search_url: String,
    ///
    issues_url: String,
    ///
    keys_url: String,
    ///
    label_search_url: String,
    ///
    notifications_url: String,
    ///
    organization_url: String,
    ///
    organization_repositories_url: String,
    ///
    organization_teams_url: String,
    ///
    public_gists_url: String,
    ///
    rate_limit_url: String,
    ///
    repository_url: String,
    ///
    repository_search_url: String,
    ///
    current_user_repositories_url: String,
    ///
    starred_url: String,
    ///
    starred_gists_url: String,
    ///
    topic_search_url: String,
    ///
    user_url: String,
    ///
    user_organizations_url: String,
    ///
    user_repositories_url: String,
    ///
    user_search_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAppResponse {
    /// * example - 37
    id: i64,
    /// * example - probot-owners
    slug: String,
    /// * example - MDExOkludGVncmF0aW9uMQ==
    node_id: String,
    /// Simple User
    owner: Value,
    /// * example - Probot Owners
    name: String,
    /// * example - The description of the app.
    description: String,
    /// * example - https://example.com
    external_url: String,
    /// * example - https://github.com/apps/super-ci
    html_url: String,
    /// * example - 2017-07-08T16:18:44-04:00
    created_at: String,
    /// * example - 2017-07-08T16:18:44-04:00
    updated_at: String,
    /// * example - [object Object]
    permissions: Value,
    /// * example - label,deployment
    events: Vec<String>,
    /// * example - 5
    installations_count: i64,
    /// * example - "Iv1.25b5d1e65ffc4022"
    client_id: String,
    /// * example - "1d4b2097ac622ba702d19de498f005747a8b21d3"
    client_secret: String,
    /// * example - "6fba8f2fc8a7e8f2cca5577eddd82ca7586b3b6b"
    webhook_secret: String,
    /// * example - "-----BEGIN RSA PRIVATE KEY-----\nMIIEogIBAAKCAQEArYxrNYD/iT5CZVpRJu4rBKmmze3PVmT/gCo2ATUvDvZTPTey\nxcGJ3vvrJXazKk06pN05TN29o98jrYz4cengG3YGsXPNEpKsIrEl8NhbnxapEnM9\nJCMRe0P5JcPsfZlX6hmiT7136GRWiGOUba2X9+HKh8QJVLG5rM007TBER9/z9mWm\nrJuNh+m5l320oBQY/Qq3A7wzdEfZw8qm/mIN0FCeoXH1L6B8xXWaAYBwhTEh6SSn\nZHlO1Xu1JWDmAvBCi0RO5aRSKM8q9QEkvvHP4yweAtK3N8+aAbZ7ovaDhyGz8r6r\nzhU1b8Uo0Z2ysf503WqzQgIajr7Fry7/kUwpgQIDAQABAoIBADwJp80Ko1xHPZDy\nfcCKBDfIuPvkmSW6KumbsLMaQv1aGdHDwwTGv3t0ixSay8CGlxMRtRDyZPib6SvQ\n6OH/lpfpbMdW2ErkksgtoIKBVrDilfrcAvrNZu7NxRNbhCSvN8q0s4ICecjbbVQh\nnueSdlA6vGXbW58BHMq68uRbHkP+k+mM9U0mDJ1HMch67wlg5GbayVRt63H7R2+r\nVxcna7B80J/lCEjIYZznawgiTvp3MSanTglqAYi+m1EcSsP14bJIB9vgaxS79kTu\noiSo93leJbBvuGo8QEiUqTwMw4tDksmkLsoqNKQ1q9P7LZ9DGcujtPy4EZsamSJT\ny8OJt0ECgYEA2lxOxJsQk2kI325JgKFjo92mQeUObIvPfSNWUIZQDTjniOI6Gv63\nGLWVFrZcvQBWjMEQraJA9xjPbblV8PtfO87MiJGLWCHFxmPz2dzoedN+2Coxom8m\nV95CLz8QUShuao6u/RYcvUaZEoYs5bHcTmy5sBK80JyEmafJPtCQVxMCgYEAy3ar\nZr3yv4xRPEPMat4rseswmuMooSaK3SKub19WFI5IAtB/e7qR1Rj9JhOGcZz+OQrl\nT78O2OFYlgOIkJPvRMrPpK5V9lslc7tz1FSh3BZMRGq5jSyD7ETSOQ0c8T2O/s7v\nbeEPbVbDe4mwvM24XByH0GnWveVxaDl51ABD65sCgYB3ZAspUkOA5egVCh8kNpnd\nSd6SnuQBE3ySRlT2WEnCwP9Ph6oPgn+oAfiPX4xbRqkL8q/k0BdHQ4h+zNwhk7+h\nWtPYRAP1Xxnc/F+jGjb+DVaIaKGU18MWPg7f+FI6nampl3Q0KvfxwX0GdNhtio8T\nTj1E+SnFwh56SRQuxSh2gwKBgHKjlIO5NtNSflsUYFM+hyQiPiqnHzddfhSG+/3o\nm5nNaSmczJesUYreH5San7/YEy2UxAugvP7aSY2MxB+iGsiJ9WD2kZzTUlDZJ7RV\nUzWsoqBR+eZfVJ2FUWWvy8TpSG6trh4dFxImNtKejCR1TREpSiTV3Zb1dmahK9GV\nrK9NAoGAbBxRLoC01xfxCTgt5BDiBcFVh4fp5yYKwavJPLzHSpuDOrrI9jDn1oKN\nonq5sDU1i391zfQvdrbX4Ova48BN+B7p63FocP/MK5tyyBoT8zQEk2+vWDOw7H/Z\nu5dTCPxTIsoIwUw1I+7yIxqJzLPFgR2gVBwY1ra/8iAqCj+zeBw=\n-----END RSA PRIVATE KEY-----\n"
    pem: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAppHookConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchAppHookConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAppHookDeliveriesdeliveryIdResponse {
    /// * example - 42
    id: i64,
    /// * example - 58474f00-b361-11eb-836d-0e4f3503ccbe
    guid: String,
    /// * example - 2021-05-12T20:33:44Z
    delivered_at: String,
    /// Whether the delivery is a redelivery.
    redelivery: bool,
    /// * example - 0.03
    duration: f64,
    /// * example - failed to connect
    status: String,
    /// * example - 502
    status_code: i64,
    /// * example - issues
    event: String,
    /// * example - opened
    action: String,
    /// * example - 123
    installation_id: i64,
    /// * example - 123
    repository_id: i64,
    /// * example - https://www.example.com
    url: String,
    ///
    request: Value,
    ///
    response: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAppInstallationsinstallationIdResponse {
    /// * example - 1
    id: i64,
    /// Describe whether all repositories have been selected or there's a selection involved
    repository_selection: String,
    /// * example - https://api.github.com/installations/1/access_tokens
    access_tokens_url: String,
    /// * example - https://api.github.com/installation/repositories
    repositories_url: String,
    /// * example - https://github.com/organizations/github/settings/installations/1
    html_url: String,
    /// * example - 1
    app_id: i64,
    /// The ID of the user or organization this token is being scoped to.
    target_id: i64,
    /// * example - Organization
    target_type: String,
    /// * example - [object Object]
    permissions: Value,
    ///
    events: Vec<String>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - config.yaml
    single_file_name: String,
    /// * example - true
    has_multiple_single_files: bool,
    /// * example - config.yml,.github/issue_TEMPLATE.md
    single_file_paths: Vec<String>,
    /// * example - github-actions
    app_slug: String,
    /// Simple User
    suspended_by: Value,
    ///
    suspended_at: String,
    /// * example - "test_13f1e99741e3e004@d7e1eb0bc0a1ba12.com"
    contact_email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetApplicationsGrantsgrantIdResponse {
    /// * example - 1
    id: i64,
    /// * example - https://api.github.com/applications/grants/1
    url: String,
    ///
    app: Value,
    /// * example - 2011-09-06T17:26:27Z
    created_at: String,
    /// * example - 2011-09-06T20:39:23Z
    updated_at: String,
    /// * example - public_repo
    scopes: Vec<String>,
    /// Simple User
    user: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostApplicationsclientIdTokenResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchApplicationsclientIdTokenResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostApplicationsclientIdTokenScopedResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAppsappSlugResponse {
    /// * example - 37
    id: i64,
    /// * example - probot-owners
    slug: String,
    /// * example - MDExOkludGVncmF0aW9uMQ==
    node_id: String,
    /// Simple User
    owner: Value,
    /// * example - Probot Owners
    name: String,
    /// * example - The description of the app.
    description: String,
    /// * example - https://example.com
    external_url: String,
    /// * example - https://github.com/apps/super-ci
    html_url: String,
    /// * example - 2017-07-08T16:18:44-04:00
    created_at: String,
    /// * example - 2017-07-08T16:18:44-04:00
    updated_at: String,
    /// * example - [object Object]
    permissions: Value,
    /// * example - label,deployment
    events: Vec<String>,
    /// * example - 5
    installations_count: i64,
    /// * example - "Iv1.25b5d1e65ffc4022"
    client_id: String,
    /// * example - "1d4b2097ac622ba702d19de498f005747a8b21d3"
    client_secret: String,
    /// * example - "6fba8f2fc8a7e8f2cca5577eddd82ca7586b3b6b"
    webhook_secret: String,
    /// * example - "-----BEGIN RSA PRIVATE KEY-----\nMIIEogIBAAKCAQEArYxrNYD/iT5CZVpRJu4rBKmmze3PVmT/gCo2ATUvDvZTPTey\nxcGJ3vvrJXazKk06pN05TN29o98jrYz4cengG3YGsXPNEpKsIrEl8NhbnxapEnM9\nJCMRe0P5JcPsfZlX6hmiT7136GRWiGOUba2X9+HKh8QJVLG5rM007TBER9/z9mWm\nrJuNh+m5l320oBQY/Qq3A7wzdEfZw8qm/mIN0FCeoXH1L6B8xXWaAYBwhTEh6SSn\nZHlO1Xu1JWDmAvBCi0RO5aRSKM8q9QEkvvHP4yweAtK3N8+aAbZ7ovaDhyGz8r6r\nzhU1b8Uo0Z2ysf503WqzQgIajr7Fry7/kUwpgQIDAQABAoIBADwJp80Ko1xHPZDy\nfcCKBDfIuPvkmSW6KumbsLMaQv1aGdHDwwTGv3t0ixSay8CGlxMRtRDyZPib6SvQ\n6OH/lpfpbMdW2ErkksgtoIKBVrDilfrcAvrNZu7NxRNbhCSvN8q0s4ICecjbbVQh\nnueSdlA6vGXbW58BHMq68uRbHkP+k+mM9U0mDJ1HMch67wlg5GbayVRt63H7R2+r\nVxcna7B80J/lCEjIYZznawgiTvp3MSanTglqAYi+m1EcSsP14bJIB9vgaxS79kTu\noiSo93leJbBvuGo8QEiUqTwMw4tDksmkLsoqNKQ1q9P7LZ9DGcujtPy4EZsamSJT\ny8OJt0ECgYEA2lxOxJsQk2kI325JgKFjo92mQeUObIvPfSNWUIZQDTjniOI6Gv63\nGLWVFrZcvQBWjMEQraJA9xjPbblV8PtfO87MiJGLWCHFxmPz2dzoedN+2Coxom8m\nV95CLz8QUShuao6u/RYcvUaZEoYs5bHcTmy5sBK80JyEmafJPtCQVxMCgYEAy3ar\nZr3yv4xRPEPMat4rseswmuMooSaK3SKub19WFI5IAtB/e7qR1Rj9JhOGcZz+OQrl\nT78O2OFYlgOIkJPvRMrPpK5V9lslc7tz1FSh3BZMRGq5jSyD7ETSOQ0c8T2O/s7v\nbeEPbVbDe4mwvM24XByH0GnWveVxaDl51ABD65sCgYB3ZAspUkOA5egVCh8kNpnd\nSd6SnuQBE3ySRlT2WEnCwP9Ph6oPgn+oAfiPX4xbRqkL8q/k0BdHQ4h+zNwhk7+h\nWtPYRAP1Xxnc/F+jGjb+DVaIaKGU18MWPg7f+FI6nampl3Q0KvfxwX0GdNhtio8T\nTj1E+SnFwh56SRQuxSh2gwKBgHKjlIO5NtNSflsUYFM+hyQiPiqnHzddfhSG+/3o\nm5nNaSmczJesUYreH5San7/YEy2UxAugvP7aSY2MxB+iGsiJ9WD2kZzTUlDZJ7RV\nUzWsoqBR+eZfVJ2FUWWvy8TpSG6trh4dFxImNtKejCR1TREpSiTV3Zb1dmahK9GV\nrK9NAoGAbBxRLoC01xfxCTgt5BDiBcFVh4fp5yYKwavJPLzHSpuDOrrI9jDn1oKN\nonq5sDU1i391zfQvdrbX4Ova48BN+B7p63FocP/MK5tyyBoT8zQEk2+vWDOw7H/Z\nu5dTCPxTIsoIwUw1I+7yIxqJzLPFgR2gVBwY1ra/8iAqCj+zeBw=\n-----END RSA PRIVATE KEY-----\n"
    pem: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutAuthorizationsClientsclientIdResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutAuthorizationsClientsclientIdfingerprintResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAuthorizationsauthorizationIdResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchAuthorizationsauthorizationIdResponse {
    ///
    id: i64,
    ///
    url: String,
    /// A list of scopes that this authorization is in.
    scopes: Vec<String>,
    ///
    token: String,
    ///
    token_last_eight: String,
    ///
    hashed_token: String,
    ///
    app: Value,
    ///
    note: String,
    ///
    note_url: String,
    ///
    updated_at: String,
    ///
    created_at: String,
    ///
    fingerprint: String,
    /// Simple User
    user: Value,
    ///
    installation: Value,
    ///
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCodesOfConductkeyResponse {
    /// * example - contributor_covenant
    key: String,
    /// * example - Contributor Covenant
    name: String,
    /// * example - https://api.github.com/codes_of_conduct/contributor_covenant
    url: String,
    /// * example - # Contributor Covenant Code of Conduct
    ///
    /// ## Our Pledge
    ///
    /// In the interest of fostering an open and welcoming environment, we as contributors and maintainers pledge to making participation in our project and our community a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.
    ///
    /// ## Our Standards
    ///
    /// Examples of behavior that contributes to creating a positive environment include:
    ///
    /// * Using welcoming and inclusive language
    /// * Being respectful of differing viewpoints and experiences
    /// * Gracefully accepting constructive criticism
    /// * Focusing on what is best for the community
    /// * Showing empathy towards other community members
    ///
    /// Examples of unacceptable behavior by participants include:
    ///
    /// * The use of sexualized language or imagery and unwelcome sexual attention or advances
    /// * Trolling, insulting/derogatory comments, and personal or political attacks
    /// * Public or private harassment
    /// * Publishing others' private information, such as a physical or electronic address, without explicit permission
    /// * Other conduct which could reasonably be considered inappropriate in a professional setting
    ///
    /// ## Our Responsibilities
    ///
    /// Project maintainers are responsible for clarifying the standards of acceptable behavior and are expected to take appropriate and fair corrective action in response
    ///                   to any instances of unacceptable behavior.
    ///
    /// Project maintainers have the right and responsibility to remove, edit, or reject comments, commits, code, wiki edits, issues, and other contributions that are not aligned to this Code of Conduct, or to ban temporarily or permanently any contributor for other behaviors that they deem inappropriate, threatening, offensive, or harmful.
    ///
    /// ## Scope
    ///
    /// This Code of Conduct applies both within project spaces and in public spaces when an individual is representing the project or its community. Examples of representing a project or community include using an official project e-mail address,
    ///                   posting via an official social media account, or acting as an appointed representative at an online or offline event. Representation of a project may be further defined and clarified by project maintainers.
    ///
    /// ## Enforcement
    ///
    /// Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at [EMAIL]. The project team will review and investigate all complaints, and will respond in a way that it deems appropriate to the circumstances. The project team is obligated to maintain confidentiality with regard to the reporter of an incident. Further details of specific enforcement policies may be posted separately.
    ///
    /// Project maintainers who do not follow or enforce the Code of Conduct in good faith may face temporary or permanent repercussions as determined by other members of the project's leadership.
    ///
    /// ## Attribution
    ///
    /// This Code of Conduct is adapted from the [Contributor Covenant][homepage], version 1.4, available at [http://contributor-covenant.org/version/1/4][version]
    ///
    /// [homepage]: http://contributor-covenant.org
    /// [version]: http://contributor-covenant.org/version/1/4/
    body: String,
    ///
    html_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsPermissionsResponse {
    /// The policy that controls the organizations in the enterprise that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
    enabled_organizations: String,
    /// The API URL to use to get or set the selected organizations that are allowed to run GitHub Actions, when `enabled_organizations` is set to `selected`.
    selected_organizations_url: String,
    /// The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
    allowed_actions: String,
    /// The API URL to use to get or set the actions that are allowed to run, when `allowed_actions` is set to `selected`.
    selected_actions_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsPermissionsOrganizationsResponse {
    ///
    total_count: f64,
    ///
    organizations: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsPermissionsSelectedActionsResponse {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    github_owned_allowed: bool,
    /// Whether actions in GitHub Marketplace from verified creators are allowed. Set to `true` to allow all GitHub Marketplace actions by verified creators.
    verified_allowed: bool,
    /// Specifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/*`."
    patterns_allowed: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnerGroupsResponse {
    ///
    total_count: f64,
    ///
    runner_groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse {
    ///
    id: f64,
    ///
    name: String,
    ///
    visibility: String,
    ///
    default: bool,
    ///
    selected_organizations_url: String,
    ///
    runners_url: String,
    ///
    allows_public_repositories: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse {
    ///
    id: f64,
    ///
    name: String,
    ///
    visibility: String,
    ///
    default: bool,
    ///
    selected_organizations_url: String,
    ///
    runners_url: String,
    ///
    allows_public_repositories: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsResponse {
    ///
    total_count: f64,
    ///
    organizations: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersResponse {
    ///
    total_count: f64,
    ///
    runners: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnersResponse {
    ///
    total_count: f64,
    ///
    runners: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnersrunnerIdResponse {
    /// * example - 5
    id: i64,
    /// * example - iMac
    name: String,
    /// * example - macos
    os: String,
    /// * example - online
    status: String,
    ///
    busy: bool,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsnameResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseSettingsBillingActionsResponse {
    /// The sum of the free and paid GitHub Actions minutes used.
    total_minutes_used: i64,
    /// The total paid GitHub Actions minutes used.
    total_paid_minutes_used: i64,
    /// The amount of free GitHub Actions minutes available.
    included_minutes: i64,
    ///
    minutes_used_breakdown: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseSettingsBillingAdvancedSecurityResponse {
    /// * example - 25
    total_advanced_security_committers: i64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseSettingsBillingPackagesResponse {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    total_gigabytes_bandwidth_used: i64,
    /// Total paid storage space (GB) for GitHuub Packages.
    total_paid_gigabytes_bandwidth_used: i64,
    /// Free storage space (GB) for GitHub Packages.
    included_gigabytes_bandwidth: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEnterprisesenterpriseSettingsBillingSharedStorageResponse {
    /// Numbers of days left in billing cycle.
    days_left_in_billing_cycle: i64,
    /// Estimated storage space (GB) used in billing cycle.
    estimated_paid_storage_for_month: i64,
    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    estimated_storage_for_month: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetFeedsResponse {
    /// * example - https://github.com/timeline
    timeline_url: String,
    /// * example - https://github.com/{user}
    user_url: String,
    /// * example - https://github.com/octocat
    current_user_public_url: String,
    /// * example - https://github.com/octocat.private?token=abc123
    current_user_url: String,
    /// * example - https://github.com/octocat.private.actor?token=abc123
    current_user_actor_url: String,
    /// * example - https://github.com/octocat-org
    current_user_organization_url: String,
    /// * example - https://github.com/organizations/github/octocat.private.atom?token=abc123
    current_user_organization_urls: Vec<String>,
    /// * example - https://github.com/security-advisories
    security_advisories_url: String,
    ///
    _links: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGistsgistIdResponse {
    ///
    forks: Vec<Value>,
    ///
    history: Vec<Value>,
    /// Gist
    fork_of: Value,
    ///
    url: String,
    ///
    forks_url: String,
    ///
    commits_url: String,
    ///
    id: String,
    ///
    node_id: String,
    ///
    git_pull_url: String,
    ///
    git_push_url: String,
    ///
    html_url: String,
    ///
    files: Value,
    ///
    public: bool,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    description: String,
    ///
    comments: i64,
    ///
    user: String,
    ///
    comments_url: String,
    /// Simple User
    owner: Value,
    ///
    truncated: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchGistsgistIdResponse {
    ///
    forks: Vec<Value>,
    ///
    history: Vec<Value>,
    /// Gist
    fork_of: Value,
    ///
    url: String,
    ///
    forks_url: String,
    ///
    commits_url: String,
    ///
    id: String,
    ///
    node_id: String,
    ///
    git_pull_url: String,
    ///
    git_push_url: String,
    ///
    html_url: String,
    ///
    files: Value,
    ///
    public: bool,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    description: String,
    ///
    comments: i64,
    ///
    user: String,
    ///
    comments_url: String,
    /// Simple User
    owner: Value,
    ///
    truncated: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGistsgistIdCommentscommentIdResponse {
    /// * example - 1
    id: i64,
    /// * example - MDExOkdpc3RDb21tZW50MQ==
    node_id: String,
    /// * example - https://api.github.com/gists/a6db0bec360bb87e9418/comments/1
    url: String,
    /// * example - Body of the attachment
    body: String,
    /// Simple User
    user: Value,
    /// * example - 2011-04-18T23:23:56Z
    created_at: String,
    /// * example - 2011-04-18T23:23:56Z
    updated_at: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchGistsgistIdCommentscommentIdResponse {
    /// * example - 1
    id: i64,
    /// * example - MDExOkdpc3RDb21tZW50MQ==
    node_id: String,
    /// * example - https://api.github.com/gists/a6db0bec360bb87e9418/comments/1
    url: String,
    /// * example - Body of the attachment
    body: String,
    /// Simple User
    user: Value,
    /// * example - 2011-04-18T23:23:56Z
    created_at: String,
    /// * example - 2011-04-18T23:23:56Z
    updated_at: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGistsgistIdshaResponse {
    ///
    forks: Vec<Value>,
    ///
    history: Vec<Value>,
    /// Gist
    fork_of: Value,
    ///
    url: String,
    ///
    forks_url: String,
    ///
    commits_url: String,
    ///
    id: String,
    ///
    node_id: String,
    ///
    git_pull_url: String,
    ///
    git_push_url: String,
    ///
    html_url: String,
    ///
    files: Value,
    ///
    public: bool,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    description: String,
    ///
    comments: i64,
    ///
    user: String,
    ///
    comments_url: String,
    /// Simple User
    owner: Value,
    ///
    truncated: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetGitignoreTemplatesnameResponse {
    /// * example - C
    name: String,
    /// * example - # Object files
    /// *.o
    ///
    /// # Libraries
    /// *.lib
    /// *.a
    ///
    /// # Shared objects (inc. Windows DLLs)
    /// *.dll
    /// *.so
    /// *.so.*
    /// *.dylib
    ///
    /// # Executables
    /// *.exe
    /// *.out
    /// *.app
    source: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetInstallationRepositoriesResponse {
    ///
    total_count: i64,
    ///
    repositories: Vec<Value>,
    /// * example - selected
    repository_selection: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetLicenseslicenseResponse {
    /// * example - mit
    key: String,
    /// * example - MIT License
    name: String,
    /// * example - MIT
    spdx_id: String,
    /// * example - https://api.github.com/licenses/mit
    url: String,
    /// * example - MDc6TGljZW5zZW1pdA==
    node_id: String,
    /// * example - http://choosealicense.com/licenses/mit/
    html_url: String,
    /// * example - A permissive license that is short and to the point. It lets people do anything with your code with proper attribution and without warranty.
    description: String,
    /// * example - Create a text file (typically named LICENSE or LICENSE.txt) in the root of your source code and copy the text of the license into the file. Replace [year] with the current year and [fullname] with the name (or names) of the copyright holders.
    implementation: String,
    /// * example - commercial-use,modifications,distribution,sublicense,private-use
    permissions: Vec<String>,
    /// * example - include-copyright
    conditions: Vec<String>,
    /// * example - no-liability
    limitations: Vec<String>,
    /// * example -
    ///
    /// The MIT License (MIT)
    ///
    /// Copyright (c) [year] [fullname]
    ///
    /// Permission is hereby granted, free of charge, to any person obtaining a copy
    /// of this software and associated documentation files (the "Software"), to deal
    /// in the Software without restriction, including without limitation the rights
    /// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    /// copies of the Software, and to permit persons to whom the Software is
    /// furnished to do so, subject to the following conditions:
    ///
    /// The above copyright notice and this permission notice shall be included in all
    /// copies or substantial portions of the Software.
    ///
    /// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    /// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    /// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    /// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    /// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    /// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    /// SOFTWARE.
    body: String,
    /// * example - true
    featured: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMarketplaceListingAccountsaccountIdResponse {
    ///
    url: String,
    ///
    #[serde(rename = "type")]
    atype: String,
    ///
    id: i64,
    ///
    login: String,
    ///
    organization_billing_email: String,
    ///
    email: String,
    ///
    marketplace_pending_change: Value,
    ///
    marketplace_purchase: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMarketplaceListingStubbedAccountsaccountIdResponse {
    ///
    url: String,
    ///
    #[serde(rename = "type")]
    atype: String,
    ///
    id: i64,
    ///
    login: String,
    ///
    organization_billing_email: String,
    ///
    email: String,
    ///
    marketplace_pending_change: Value,
    ///
    marketplace_purchase: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMetaResponse {
    /// * example - true
    verifiable_password_authentication: bool,
    ///
    ssh_key_fingerprints: Value,
    /// * example - 127.0.0.1/32
    hooks: Vec<String>,
    /// * example - 127.0.0.1/32
    web: Vec<String>,
    /// * example - 127.0.0.1/32
    api: Vec<String>,
    /// * example - 127.0.0.1/32
    git: Vec<String>,
    /// * example - 13.65.0.0/16,157.55.204.33/32,2a01:111:f403:f90c::/62
    packages: Vec<String>,
    /// * example - 192.30.252.153/32,192.30.252.154/32
    pages: Vec<String>,
    /// * example - 54.158.161.132,54.226.70.38
    importer: Vec<String>,
    /// * example - 13.64.0.0/16,13.65.0.0/16
    actions: Vec<String>,
    /// * example - 192.168.7.15/32,192.168.7.16/32
    dependabot: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetNotificationsThreadsthreadIdResponse {
    ///
    id: String,
    /// Minimal Repository
    repository: Value,
    ///
    subject: Value,
    ///
    reason: String,
    ///
    unread: bool,
    ///
    updated_at: String,
    ///
    last_read_at: String,
    ///
    url: String,
    /// * example - https://api.github.com/notifications/threads/2/subscription
    subscription_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetNotificationsThreadsthreadIdSubscriptionResponse {
    /// * example - true
    subscribed: bool,
    ///
    ignored: bool,
    ///
    reason: String,
    /// * example - 2012-10-06T21:34:12Z
    created_at: String,
    /// * example - https://api.github.com/notifications/threads/1/subscription
    url: String,
    /// * example - https://api.github.com/notifications/threads/1
    thread_url: String,
    /// * example - https://api.github.com/repos/1
    repository_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutNotificationsThreadsthreadIdSubscriptionResponse {
    /// * example - true
    subscribed: bool,
    ///
    ignored: bool,
    ///
    reason: String,
    /// * example - 2012-10-06T21:34:12Z
    created_at: String,
    /// * example - https://api.github.com/notifications/threads/1/subscription
    url: String,
    /// * example - https://api.github.com/notifications/threads/1
    thread_url: String,
    /// * example - https://api.github.com/repos/1
    repository_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrganizationsorganizationIdCustomRolesResponse {
    /// * example - 3
    total_count: i64,
    ///
    custom_roles: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgResponse {
    /// * example - github
    login: String,
    /// * example - 1
    id: i64,
    /// * example - MDEyOk9yZ2FuaXphdGlvbjE=
    node_id: String,
    /// * example - https://api.github.com/orgs/github
    url: String,
    /// * example - https://api.github.com/orgs/github/repos
    repos_url: String,
    /// * example - https://api.github.com/orgs/github/events
    events_url: String,
    /// * example - https://api.github.com/orgs/github/hooks
    hooks_url: String,
    /// * example - https://api.github.com/orgs/github/issues
    issues_url: String,
    /// * example - https://api.github.com/orgs/github/members{/member}
    members_url: String,
    /// * example - https://api.github.com/orgs/github/public_members{/member}
    public_members_url: String,
    /// * example - https://github.com/images/error/octocat_happy.gif
    avatar_url: String,
    /// * example - A great organization
    description: String,
    /// * example - github
    name: String,
    /// * example - GitHub
    company: String,
    /// * example - https://github.com/blog
    blog: String,
    /// * example - San Francisco
    location: String,
    /// * example - octocat@github.com
    email: String,
    /// * example - github
    twitter_username: String,
    /// * example - true
    is_verified: bool,
    /// * example - true
    has_organization_projects: bool,
    /// * example - true
    has_repository_projects: bool,
    /// * example - 2
    public_repos: i64,
    /// * example - 1
    public_gists: i64,
    /// * example - 20
    followers: i64,
    ///
    following: i64,
    /// * example - https://github.com/octocat
    html_url: String,
    /// * example - 2008-01-14T04:33:35Z
    created_at: String,
    /// * example - Organization
    #[serde(rename = "type")]
    atype: String,
    /// * example - 100
    total_private_repos: i64,
    /// * example - 100
    owned_private_repos: i64,
    /// * example - 81
    private_gists: i64,
    /// * example - 10000
    disk_usage: i64,
    /// * example - 8
    collaborators: i64,
    /// * example - org@example.com
    billing_email: String,
    ///
    plan: Value,
    ///
    default_repository_permission: String,
    /// * example - true
    members_can_create_repositories: bool,
    /// * example - true
    two_factor_requirement_enabled: bool,
    /// * example - all
    members_allowed_repository_creation_type: String,
    /// * example - true
    members_can_create_public_repositories: bool,
    /// * example - true
    members_can_create_private_repositories: bool,
    /// * example - true
    members_can_create_internal_repositories: bool,
    /// * example - true
    members_can_create_pages: bool,
    /// * example - true
    members_can_create_public_pages: bool,
    /// * example - true
    members_can_create_private_pages: bool,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgResponse {
    /// * example - github
    login: String,
    /// * example - 1
    id: i64,
    /// * example - MDEyOk9yZ2FuaXphdGlvbjE=
    node_id: String,
    /// * example - https://api.github.com/orgs/github
    url: String,
    /// * example - https://api.github.com/orgs/github/repos
    repos_url: String,
    /// * example - https://api.github.com/orgs/github/events
    events_url: String,
    /// * example - https://api.github.com/orgs/github/hooks
    hooks_url: String,
    /// * example - https://api.github.com/orgs/github/issues
    issues_url: String,
    /// * example - https://api.github.com/orgs/github/members{/member}
    members_url: String,
    /// * example - https://api.github.com/orgs/github/public_members{/member}
    public_members_url: String,
    /// * example - https://github.com/images/error/octocat_happy.gif
    avatar_url: String,
    /// * example - A great organization
    description: String,
    /// * example - github
    name: String,
    /// * example - GitHub
    company: String,
    /// * example - https://github.com/blog
    blog: String,
    /// * example - San Francisco
    location: String,
    /// * example - octocat@github.com
    email: String,
    /// * example - github
    twitter_username: String,
    /// * example - true
    is_verified: bool,
    /// * example - true
    has_organization_projects: bool,
    /// * example - true
    has_repository_projects: bool,
    /// * example - 2
    public_repos: i64,
    /// * example - 1
    public_gists: i64,
    /// * example - 20
    followers: i64,
    ///
    following: i64,
    /// * example - https://github.com/octocat
    html_url: String,
    /// * example - 2008-01-14T04:33:35Z
    created_at: String,
    /// * example - Organization
    #[serde(rename = "type")]
    atype: String,
    /// * example - 100
    total_private_repos: i64,
    /// * example - 100
    owned_private_repos: i64,
    /// * example - 81
    private_gists: i64,
    /// * example - 10000
    disk_usage: i64,
    /// * example - 8
    collaborators: i64,
    /// * example - org@example.com
    billing_email: String,
    ///
    plan: Value,
    ///
    default_repository_permission: String,
    /// * example - true
    members_can_create_repositories: bool,
    /// * example - true
    two_factor_requirement_enabled: bool,
    /// * example - all
    members_allowed_repository_creation_type: String,
    /// * example - true
    members_can_create_public_repositories: bool,
    /// * example - true
    members_can_create_private_repositories: bool,
    /// * example - true
    members_can_create_internal_repositories: bool,
    /// * example - true
    members_can_create_pages: bool,
    /// * example - true
    members_can_create_public_pages: bool,
    /// * example - true
    members_can_create_private_pages: bool,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsPermissionsResponse {
    /// The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
    enabled_repositories: String,
    /// The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`.
    selected_repositories_url: String,
    /// The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
    allowed_actions: String,
    /// The API URL to use to get or set the actions that are allowed to run, when `allowed_actions` is set to `selected`.
    selected_actions_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsPermissionsRepositoriesResponse {
    ///
    total_count: f64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsPermissionsSelectedActionsResponse {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    github_owned_allowed: bool,
    /// Whether actions in GitHub Marketplace from verified creators are allowed. Set to `true` to allow all GitHub Marketplace actions by verified creators.
    verified_allowed: bool,
    /// Specifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/*`."
    patterns_allowed: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnerGroupsResponse {
    ///
    total_count: f64,
    ///
    runner_groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnerGroupsrunnerGroupIdResponse {
    ///
    id: f64,
    ///
    name: String,
    ///
    visibility: String,
    ///
    default: bool,
    /// Link to the selected repositories resource for this runner group. Not present unless visibility was set to `selected`
    selected_repositories_url: String,
    ///
    runners_url: String,
    ///
    inherited: bool,
    ///
    inherited_allows_public_repositories: bool,
    ///
    allows_public_repositories: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgActionsRunnerGroupsrunnerGroupIdResponse {
    ///
    id: f64,
    ///
    name: String,
    ///
    visibility: String,
    ///
    default: bool,
    /// Link to the selected repositories resource for this runner group. Not present unless visibility was set to `selected`
    selected_repositories_url: String,
    ///
    runners_url: String,
    ///
    inherited: bool,
    ///
    inherited_allows_public_repositories: bool,
    ///
    allows_public_repositories: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesResponse {
    ///
    total_count: f64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersResponse {
    ///
    total_count: f64,
    ///
    runners: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnersResponse {
    ///
    total_count: i64,
    ///
    runners: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnersrunnerIdResponse {
    /// * example - 5
    id: i64,
    /// * example - iMac
    name: String,
    /// * example - macos
    os: String,
    /// * example - online
    status: String,
    ///
    busy: bool,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostOrgsorgActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutOrgsorgActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteOrgsorgActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteOrgsorgActionsRunnersrunnerIdLabelsnameResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsSecretsResponse {
    ///
    total_count: i64,
    ///
    secrets: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsSecretsPublicKeyResponse {
    /// * example - 1234567
    key_id: String,
    /// * example - hBT5WZEj8ZoOv6TYJsfWq7MxTEQopZO5/IT3ZCVQPzs=
    key: String,
    /// * example - 2
    id: i64,
    /// * example - https://api.github.com/user/keys/2
    url: String,
    /// * example - ssh-rsa AAAAB3NzaC1yc2EAAA
    title: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsSecretssecretNameResponse {
    /// * example - SECRET_TOKEN
    name: String,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Visibility of a secret
    visibility: String,
    /// * example - https://api.github.com/organizations/org/secrets/my_secret/repositories
    selected_repositories_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgActionsSecretssecretNameRepositoriesResponse {
    ///
    total_count: i64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgExternalGroupgroupIdResponse {
    /// * example - 1
    group_id: i64,
    /// * example - group-azuread-test
    group_name: String,
    /// * example - 2021-01-03 22:27:15:000 -700
    updated_at: String,
    /// * example - [object Object],[object Object]
    teams: Vec<Value>,
    /// * example - [object Object],[object Object]
    members: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgExternalGroupsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgHookshookIdResponse {
    /// * example - 1
    id: i64,
    /// * example - https://api.github.com/orgs/octocat/hooks/1
    url: String,
    /// * example - https://api.github.com/orgs/octocat/hooks/1/pings
    ping_url: String,
    /// * example - https://api.github.com/orgs/octocat/hooks/1/deliveries
    deliveries_url: String,
    /// * example - web
    name: String,
    /// * example - push,pull_request
    events: Vec<String>,
    /// * example - true
    active: bool,
    ///
    config: Value,
    /// * example - 2011-09-06T20:39:23Z
    updated_at: String,
    /// * example - 2011-09-06T17:26:27Z
    created_at: String,
    ///
    #[serde(rename = "type")]
    atype: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgHookshookIdResponse {
    /// * example - 1
    id: i64,
    /// * example - https://api.github.com/orgs/octocat/hooks/1
    url: String,
    /// * example - https://api.github.com/orgs/octocat/hooks/1/pings
    ping_url: String,
    /// * example - https://api.github.com/orgs/octocat/hooks/1/deliveries
    deliveries_url: String,
    /// * example - web
    name: String,
    /// * example - push,pull_request
    events: Vec<String>,
    /// * example - true
    active: bool,
    ///
    config: Value,
    /// * example - 2011-09-06T20:39:23Z
    updated_at: String,
    /// * example - 2011-09-06T17:26:27Z
    created_at: String,
    ///
    #[serde(rename = "type")]
    atype: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgHookshookIdConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgHookshookIdConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgHookshookIdDeliveriesdeliveryIdResponse {
    /// * example - 42
    id: i64,
    /// * example - 58474f00-b361-11eb-836d-0e4f3503ccbe
    guid: String,
    /// * example - 2021-05-12T20:33:44Z
    delivered_at: String,
    /// Whether the delivery is a redelivery.
    redelivery: bool,
    /// * example - 0.03
    duration: f64,
    /// * example - failed to connect
    status: String,
    /// * example - 502
    status_code: i64,
    /// * example - issues
    event: String,
    /// * example - opened
    action: String,
    /// * example - 123
    installation_id: i64,
    /// * example - 123
    repository_id: i64,
    /// * example - https://www.example.com
    url: String,
    ///
    request: Value,
    ///
    response: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgInstallationResponse {
    /// * example - 1
    id: i64,
    /// Describe whether all repositories have been selected or there's a selection involved
    repository_selection: String,
    /// * example - https://api.github.com/installations/1/access_tokens
    access_tokens_url: String,
    /// * example - https://api.github.com/installation/repositories
    repositories_url: String,
    /// * example - https://github.com/organizations/github/settings/installations/1
    html_url: String,
    /// * example - 1
    app_id: i64,
    /// The ID of the user or organization this token is being scoped to.
    target_id: i64,
    /// * example - Organization
    target_type: String,
    /// * example - [object Object]
    permissions: Value,
    ///
    events: Vec<String>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - config.yaml
    single_file_name: String,
    /// * example - true
    has_multiple_single_files: bool,
    /// * example - config.yml,.github/issue_TEMPLATE.md
    single_file_paths: Vec<String>,
    /// * example - github-actions
    app_slug: String,
    /// Simple User
    suspended_by: Value,
    ///
    suspended_at: String,
    /// * example - "test_13f1e99741e3e004@d7e1eb0bc0a1ba12.com"
    contact_email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgInstallationsResponse {
    ///
    total_count: i64,
    ///
    installations: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutOrgsorgInteractionLimitsResponse {
    /// * example - collaborators_only
    limit: String,
    /// * example - repository
    origin: String,
    /// * example - 2018-08-17T04:18:39Z
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgMembershipsusernameResponse {
    /// * example - https://api.github.com/orgs/octocat/memberships/defunkt
    url: String,
    /// * example - active
    state: String,
    /// * example - admin
    role: String,
    /// * example - https://api.github.com/orgs/octocat
    organization_url: String,
    /// Organization Simple
    organization: Value,
    /// Simple User
    user: Value,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutOrgsorgMembershipsusernameResponse {
    /// * example - https://api.github.com/orgs/octocat/memberships/defunkt
    url: String,
    /// * example - active
    state: String,
    /// * example - admin
    role: String,
    /// * example - https://api.github.com/orgs/octocat
    organization_url: String,
    /// Organization Simple
    organization: Value,
    /// Simple User
    user: Value,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgMigrationsmigrationIdResponse {
    /// * example - 79
    id: i64,
    /// Simple User
    owner: Value,
    /// * example - 0b989ba4-242f-11e5-81e1-c7b6966d2516
    guid: String,
    /// * example - pending
    state: String,
    /// * example - true
    lock_repositories: bool,
    ///
    exclude_metadata: bool,
    ///
    exclude_git_data: bool,
    ///
    exclude_attachments: bool,
    ///
    exclude_releases: bool,
    ///
    exclude_owner_projects: bool,
    ///
    repositories: Vec<Value>,
    /// * example - https://api.github.com/orgs/octo-org/migrations/79
    url: String,
    /// * example - 2015-07-06T15:33:38-07:00
    created_at: String,
    /// * example - 2015-07-06T15:33:38-07:00
    updated_at: String,
    ///
    node_id: String,
    ///
    archive_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgPackagespackageTypepackageNameResponse {
    /// * example - 1
    id: i64,
    /// * example - super-linter
    name: String,
    /// * example - docker
    package_type: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    html_url: String,
    /// * example - 1
    version_count: i64,
    /// * example - private
    visibility: String,
    /// Simple User
    owner: Value,
    /// Minimal Repository
    repository: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdResponse {
    /// * example - 1
    id: i64,
    /// * example - latest
    name: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter/versions/786068
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    package_html_url: String,
    /// * example - https://github.com/orgs/github/packages/container/super-linter/786068
    html_url: String,
    /// * example - MIT
    license: String,
    ///
    description: String,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// * example - 2014-03-03T18:58:10Z
    deleted_at: String,
    ///
    metadata: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgSettingsBillingActionsResponse {
    /// The sum of the free and paid GitHub Actions minutes used.
    total_minutes_used: i64,
    /// The total paid GitHub Actions minutes used.
    total_paid_minutes_used: i64,
    /// The amount of free GitHub Actions minutes available.
    included_minutes: i64,
    ///
    minutes_used_breakdown: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgSettingsBillingAdvancedSecurityResponse {
    /// * example - 25
    total_advanced_security_committers: i64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgSettingsBillingPackagesResponse {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    total_gigabytes_bandwidth_used: i64,
    /// Total paid storage space (GB) for GitHuub Packages.
    total_paid_gigabytes_bandwidth_used: i64,
    /// Free storage space (GB) for GitHub Packages.
    included_gigabytes_bandwidth: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgSettingsBillingSharedStorageResponse {
    /// Numbers of days left in billing cycle.
    days_left_in_billing_cycle: i64,
    /// Estimated storage space (GB) used in billing cycle.
    estimated_paid_storage_for_month: i64,
    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    estimated_storage_for_month: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamSyncGroupsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugResponse {
    /// * example - 42
    id: i64,
    /// * example - MDQ6VGVhbTE=
    node_id: String,
    /// * example - https://api.github.com/organizations/1/team/1
    url: String,
    /// * example - https://github.com/orgs/rails/teams/core
    html_url: String,
    /// * example - Developers
    name: String,
    /// * example - justice-league
    slug: String,
    /// * example - A great team.
    description: String,
    /// * example - closed
    privacy: String,
    /// * example - push
    permission: String,
    /// * example - https://api.github.com/organizations/1/team/1/members{/member}
    members_url: String,
    /// * example - https://api.github.com/organizations/1/team/1/repos
    repositories_url: String,
    /// Groups of organization members that gives permissions on specified repositories.
    parent: Value,
    /// * example - 3
    members_count: i64,
    /// * example - 10
    repos_count: i64,
    /// * example - 2017-07-14T16:53:42Z
    created_at: String,
    /// * example - 2017-08-17T12:37:15Z
    updated_at: String,
    /// Organization Full
    organization: Value,
    /// * example - uid=example,ou=users,dc=github,dc=com
    ldap_dn: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse {
    /// Simple User
    author: Value,
    /// * example - Please suggest improvements to our workflow in comments.
    body: String,
    /// * example - <p>Hi! This is an area for us to collaborate as a team</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    ///
    comments_count: i64,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1/comments
    comments_url: String,
    /// * example - 2018-01-25T18:56:31Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1
    html_url: String,
    /// * example - MDE0OlRlYW1EaXNjdXNzaW9uMQ==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - true
    pinned: bool,
    /// * example - true
    private: bool,
    /// * example - https://api.github.com/organizations/1/team/2343027
    team_url: String,
    /// * example - How can we improve our workflow?
    title: String,
    /// * example - 2018-01-25T18:56:31Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse {
    /// Simple User
    author: Value,
    /// * example - Please suggest improvements to our workflow in comments.
    body: String,
    /// * example - <p>Hi! This is an area for us to collaborate as a team</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    ///
    comments_count: i64,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1/comments
    comments_url: String,
    /// * example - 2018-01-25T18:56:31Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1
    html_url: String,
    /// * example - MDE0OlRlYW1EaXNjdXNzaW9uMQ==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - true
    pinned: bool,
    /// * example - true
    private: bool,
    /// * example - https://api.github.com/organizations/1/team/2343027
    team_url: String,
    /// * example - How can we improve our workflow?
    title: String,
    /// * example - 2018-01-25T18:56:31Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse {
    /// Simple User
    author: Value,
    /// * example - I agree with this suggestion.
    body: String,
    /// * example - <p>Do you like apples?</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    /// * example - 2018-01-15T23:53:58Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1
    discussion_url: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1/comments/1
    html_url: String,
    /// * example - MDIxOlRlYW1EaXNjdXNzaW9uQ29tbWVudDE=
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - 2018-01-15T23:53:58Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1/comments/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse {
    /// Simple User
    author: Value,
    /// * example - I agree with this suggestion.
    body: String,
    /// * example - <p>Do you like apples?</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    /// * example - 2018-01-15T23:53:58Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1
    discussion_url: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1/comments/1
    html_url: String,
    /// * example - MDIxOlRlYW1EaXNjdXNzaW9uQ29tbWVudDE=
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - 2018-01-15T23:53:58Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1/comments/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsResponse
{
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgTeamsteamSlugExternalGroupsResponse {
    /// * example - 1
    group_id: i64,
    /// * example - group-azuread-test
    group_name: String,
    /// * example - 2021-01-03 22:27:15:000 -700
    updated_at: String,
    /// * example - [object Object],[object Object]
    teams: Vec<Value>,
    /// * example - [object Object],[object Object]
    members: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugMembershipsusernameResponse {
    ///
    url: String,
    /// * example - member
    role: String,
    /// The state of the user's membership in the team.
    state: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutOrgsorgTeamsteamSlugMembershipsusernameResponse {
    ///
    url: String,
    /// * example - member
    role: String,
    /// The state of the user's membership in the team.
    state: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugProjectsprojectIdResponse {
    ///
    owner_url: String,
    ///
    url: String,
    ///
    html_url: String,
    ///
    columns_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    ///
    name: String,
    ///
    body: String,
    ///
    number: i64,
    ///
    state: String,
    /// Simple User
    creator: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// The organization permission for this project. Only present when owner is an organization.
    organization_permission: String,
    /// Whether the project is private or not. Only present when owner is an organization.
    private: bool,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugReposownerrepoResponse {
    /// * example - 42
    id: i64,
    /// * example - MDEwOlJlcG9zaXRvcnkxMjk2MjY5
    node_id: String,
    /// * example - Team Environment
    name: String,
    /// * example - octocat/Hello-World
    full_name: String,
    /// License Simple
    license: Value,
    ///
    forks: i64,
    ///
    permissions: Value,
    /// * example - admin
    role_name: String,
    /// Simple User
    owner: Value,
    /// Whether the repository is private or public.
    private: bool,
    /// * example - https://github.com/octocat/Hello-World
    html_url: String,
    /// * example - This your first repo!
    description: String,
    ///
    fork: bool,
    /// * example - https://api.github.com/repos/octocat/Hello-World
    url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/{archive_format}{/ref}
    archive_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/assignees{/user}
    assignees_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/blobs{/sha}
    blobs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/branches{/branch}
    branches_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/collaborators{/collaborator}
    collaborators_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/comments{/number}
    comments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/commits{/sha}
    commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/compare/{base}...{head}
    compare_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contents/{+path}
    contents_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contributors
    contributors_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/deployments
    deployments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/downloads
    downloads_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/events
    events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/forks
    forks_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/commits{/sha}
    git_commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/refs{/sha}
    git_refs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/tags{/sha}
    git_tags_url: String,
    /// * example - git:github.com/octocat/Hello-World.git
    git_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/comments{/number}
    issue_comment_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/events{/number}
    issue_events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues{/number}
    issues_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/keys{/key_id}
    keys_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/labels{/name}
    labels_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/languages
    languages_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/merges
    merges_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/milestones{/number}
    milestones_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/notifications{?since,all,participating}
    notifications_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/pulls{/number}
    pulls_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/releases{/id}
    releases_url: String,
    /// * example - git@github.com:octocat/Hello-World.git
    ssh_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/stargazers
    stargazers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/statuses/{sha}
    statuses_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscribers
    subscribers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscription
    subscription_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/tags
    tags_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/teams
    teams_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/trees{/sha}
    trees_url: String,
    /// * example - https://github.com/octocat/Hello-World.git
    clone_url: String,
    /// * example - git:git.example.com/octocat/Hello-World
    mirror_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/hooks
    hooks_url: String,
    /// * example - https://svn.github.com/octocat/Hello-World
    svn_url: String,
    /// * example - https://github.com
    homepage: String,
    ///
    language: String,
    /// * example - 9
    forks_count: i64,
    /// * example - 80
    stargazers_count: i64,
    /// * example - 80
    watchers_count: i64,
    /// * example - 108
    size: i64,
    /// * example - master
    default_branch: String,
    ///
    open_issues_count: i64,
    /// * example - true
    is_template: bool,
    ///
    topics: Vec<String>,
    /// * example - true
    has_issues: bool,
    /// * example - true
    has_projects: bool,
    /// * example - true
    has_wiki: bool,
    ///
    has_pages: bool,
    /// * example - true
    has_downloads: bool,
    /// Whether the repository is archived.
    archived: bool,
    /// Returns whether or not this repository disabled.
    disabled: bool,
    /// The repository visibility: public, private, or internal.
    visibility: String,
    /// * example - 2011-01-26T19:06:43Z
    pushed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:14:43Z
    updated_at: String,
    /// * example - true
    allow_rebase_merge: bool,
    /// A git repository
    template_repository: Value,
    ///
    temp_clone_token: String,
    /// * example - true
    allow_squash_merge: bool,
    /// Whether to allow Auto-merge to be used on pull requests.
    allow_auto_merge: bool,
    /// Whether to delete head branches when pull requests are merged
    delete_branch_on_merge: bool,
    /// * example - true
    allow_merge_commit: bool,
    /// Whether to allow forking this repo
    allow_forking: bool,
    ///
    subscribers_count: i64,
    ///
    network_count: i64,
    ///
    open_issues: i64,
    ///
    watchers: i64,
    ///
    master_branch: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetProjectsColumnsCardscardIdResponse {
    /// * example - https://api.github.com/projects/columns/cards/1478
    url: String,
    /// * example - 42
    id: i64,
    /// * example - MDExOlByb2plY3RDYXJkMTQ3OA==
    node_id: String,
    /// * example - Add payload for delete Project column
    note: String,
    /// Simple User
    creator: Value,
    /// * example - 2016-09-05T14:21:06Z
    created_at: String,
    /// * example - 2016-09-05T14:20:22Z
    updated_at: String,
    /// Whether or not the card is archived
    archived: bool,
    ///
    column_name: String,
    ///
    project_id: String,
    /// * example - https://api.github.com/projects/columns/367
    column_url: String,
    /// * example - https://api.github.com/repos/api-playground/projects-test/issues/3
    content_url: String,
    /// * example - https://api.github.com/projects/120
    project_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchProjectsColumnsCardscardIdResponse {
    /// * example - https://api.github.com/projects/columns/cards/1478
    url: String,
    /// * example - 42
    id: i64,
    /// * example - MDExOlByb2plY3RDYXJkMTQ3OA==
    node_id: String,
    /// * example - Add payload for delete Project column
    note: String,
    /// Simple User
    creator: Value,
    /// * example - 2016-09-05T14:21:06Z
    created_at: String,
    /// * example - 2016-09-05T14:20:22Z
    updated_at: String,
    /// Whether or not the card is archived
    archived: bool,
    ///
    column_name: String,
    ///
    project_id: String,
    /// * example - https://api.github.com/projects/columns/367
    column_url: String,
    /// * example - https://api.github.com/repos/api-playground/projects-test/issues/3
    content_url: String,
    /// * example - https://api.github.com/projects/120
    project_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetProjectsColumnscolumnIdResponse {
    /// * example - https://api.github.com/projects/columns/367
    url: String,
    /// * example - https://api.github.com/projects/120
    project_url: String,
    /// * example - https://api.github.com/projects/columns/367/cards
    cards_url: String,
    /// * example - 42
    id: i64,
    /// * example - MDEzOlByb2plY3RDb2x1bW4zNjc=
    node_id: String,
    /// * example - Remaining tasks
    name: String,
    /// * example - 2016-09-05T14:18:44Z
    created_at: String,
    /// * example - 2016-09-05T14:22:28Z
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchProjectsColumnscolumnIdResponse {
    /// * example - https://api.github.com/projects/columns/367
    url: String,
    /// * example - https://api.github.com/projects/120
    project_url: String,
    /// * example - https://api.github.com/projects/columns/367/cards
    cards_url: String,
    /// * example - 42
    id: i64,
    /// * example - MDEzOlByb2plY3RDb2x1bW4zNjc=
    node_id: String,
    /// * example - Remaining tasks
    name: String,
    /// * example - 2016-09-05T14:18:44Z
    created_at: String,
    /// * example - 2016-09-05T14:22:28Z
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetProjectsprojectIdResponse {
    /// * example - https://api.github.com/repos/api-playground/projects-test
    owner_url: String,
    /// * example - https://api.github.com/projects/1002604
    url: String,
    /// * example - https://github.com/api-playground/projects-test/projects/12
    html_url: String,
    /// * example - https://api.github.com/projects/1002604/columns
    columns_url: String,
    /// * example - 1002604
    id: i64,
    /// * example - MDc6UHJvamVjdDEwMDI2MDQ=
    node_id: String,
    /// * example - Week One Sprint
    name: String,
    /// * example - This project represents the sprint of the first week in January
    body: String,
    /// * example - 1
    number: i64,
    /// * example - open
    state: String,
    /// Simple User
    creator: Value,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
    organization_permission: String,
    /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
    private: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchProjectsprojectIdResponse {
    /// * example - https://api.github.com/repos/api-playground/projects-test
    owner_url: String,
    /// * example - https://api.github.com/projects/1002604
    url: String,
    /// * example - https://github.com/api-playground/projects-test/projects/12
    html_url: String,
    /// * example - https://api.github.com/projects/1002604/columns
    columns_url: String,
    /// * example - 1002604
    id: i64,
    /// * example - MDc6UHJvamVjdDEwMDI2MDQ=
    node_id: String,
    /// * example - Week One Sprint
    name: String,
    /// * example - This project represents the sprint of the first week in January
    body: String,
    /// * example - 1
    number: i64,
    /// * example - open
    state: String,
    /// Simple User
    creator: Value,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
    organization_permission: String,
    /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
    private: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetProjectsprojectIdCollaboratorsusernamePermissionResponse {
    ///
    permission: String,
    /// Simple User
    user: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRateLimitResponse {
    ///
    resources: Value,
    ///
    rate: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoResponse {
    /// * example - 1296269
    id: i64,
    /// * example - MDEwOlJlcG9zaXRvcnkxMjk2MjY5
    node_id: String,
    /// * example - Hello-World
    name: String,
    /// * example - octocat/Hello-World
    full_name: String,
    /// Simple User
    owner: Value,
    ///
    private: bool,
    /// * example - https://github.com/octocat/Hello-World
    html_url: String,
    /// * example - This your first repo!
    description: String,
    ///
    fork: bool,
    /// * example - https://api.github.com/repos/octocat/Hello-World
    url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/{archive_format}{/ref}
    archive_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/assignees{/user}
    assignees_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/blobs{/sha}
    blobs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/branches{/branch}
    branches_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/collaborators{/collaborator}
    collaborators_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/comments{/number}
    comments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/commits{/sha}
    commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/compare/{base}...{head}
    compare_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contents/{+path}
    contents_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contributors
    contributors_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/deployments
    deployments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/downloads
    downloads_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/events
    events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/forks
    forks_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/commits{/sha}
    git_commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/refs{/sha}
    git_refs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/tags{/sha}
    git_tags_url: String,
    /// * example - git:github.com/octocat/Hello-World.git
    git_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/comments{/number}
    issue_comment_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/events{/number}
    issue_events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues{/number}
    issues_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/keys{/key_id}
    keys_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/labels{/name}
    labels_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/languages
    languages_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/merges
    merges_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/milestones{/number}
    milestones_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/notifications{?since,all,participating}
    notifications_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/pulls{/number}
    pulls_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/releases{/id}
    releases_url: String,
    /// * example - git@github.com:octocat/Hello-World.git
    ssh_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/stargazers
    stargazers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/statuses/{sha}
    statuses_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscribers
    subscribers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscription
    subscription_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/tags
    tags_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/teams
    teams_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/trees{/sha}
    trees_url: String,
    /// * example - https://github.com/octocat/Hello-World.git
    clone_url: String,
    /// * example - git:git.example.com/octocat/Hello-World
    mirror_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/hooks
    hooks_url: String,
    /// * example - https://svn.github.com/octocat/Hello-World
    svn_url: String,
    /// * example - https://github.com
    homepage: String,
    ///
    language: String,
    /// * example - 9
    forks_count: i64,
    /// * example - 80
    stargazers_count: i64,
    /// * example - 80
    watchers_count: i64,
    /// * example - 108
    size: i64,
    /// * example - master
    default_branch: String,
    ///
    open_issues_count: i64,
    /// * example - true
    is_template: bool,
    /// * example - octocat,atom,electron,API
    topics: Vec<String>,
    /// * example - true
    has_issues: bool,
    /// * example - true
    has_projects: bool,
    /// * example - true
    has_wiki: bool,
    ///
    has_pages: bool,
    /// * example - true
    has_downloads: bool,
    ///
    archived: bool,
    /// Returns whether or not this repository disabled.
    disabled: bool,
    /// * example - public
    visibility: String,
    /// * example - 2011-01-26T19:06:43Z
    pushed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:14:43Z
    updated_at: String,
    ///
    permissions: Value,
    /// * example - true
    allow_rebase_merge: bool,
    /// A git repository
    template_repository: Value,
    ///
    temp_clone_token: String,
    /// * example - true
    allow_squash_merge: bool,
    ///
    allow_auto_merge: bool,
    ///
    delete_branch_on_merge: bool,
    /// * example - true
    allow_merge_commit: bool,
    /// * example - true
    allow_forking: bool,
    /// * example - 42
    subscribers_count: i64,
    ///
    network_count: i64,
    /// License Simple
    license: Value,
    /// Simple User
    organization: Value,
    /// A git repository
    parent: Value,
    /// A git repository
    source: Value,
    ///
    forks: i64,
    ///
    master_branch: String,
    ///
    open_issues: i64,
    ///
    watchers: i64,
    /// Whether anonymous git access is allowed.
    anonymous_access_enabled: bool,
    /// Code of Conduct Simple
    code_of_conduct: Value,
    ///
    security_and_analysis: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoResponse {
    /// * example - 1296269
    id: i64,
    /// * example - MDEwOlJlcG9zaXRvcnkxMjk2MjY5
    node_id: String,
    /// * example - Hello-World
    name: String,
    /// * example - octocat/Hello-World
    full_name: String,
    /// Simple User
    owner: Value,
    ///
    private: bool,
    /// * example - https://github.com/octocat/Hello-World
    html_url: String,
    /// * example - This your first repo!
    description: String,
    ///
    fork: bool,
    /// * example - https://api.github.com/repos/octocat/Hello-World
    url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/{archive_format}{/ref}
    archive_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/assignees{/user}
    assignees_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/blobs{/sha}
    blobs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/branches{/branch}
    branches_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/collaborators{/collaborator}
    collaborators_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/comments{/number}
    comments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/commits{/sha}
    commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/compare/{base}...{head}
    compare_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contents/{+path}
    contents_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contributors
    contributors_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/deployments
    deployments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/downloads
    downloads_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/events
    events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/forks
    forks_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/commits{/sha}
    git_commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/refs{/sha}
    git_refs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/tags{/sha}
    git_tags_url: String,
    /// * example - git:github.com/octocat/Hello-World.git
    git_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/comments{/number}
    issue_comment_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/events{/number}
    issue_events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues{/number}
    issues_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/keys{/key_id}
    keys_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/labels{/name}
    labels_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/languages
    languages_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/merges
    merges_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/milestones{/number}
    milestones_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/notifications{?since,all,participating}
    notifications_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/pulls{/number}
    pulls_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/releases{/id}
    releases_url: String,
    /// * example - git@github.com:octocat/Hello-World.git
    ssh_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/stargazers
    stargazers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/statuses/{sha}
    statuses_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscribers
    subscribers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscription
    subscription_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/tags
    tags_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/teams
    teams_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/trees{/sha}
    trees_url: String,
    /// * example - https://github.com/octocat/Hello-World.git
    clone_url: String,
    /// * example - git:git.example.com/octocat/Hello-World
    mirror_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/hooks
    hooks_url: String,
    /// * example - https://svn.github.com/octocat/Hello-World
    svn_url: String,
    /// * example - https://github.com
    homepage: String,
    ///
    language: String,
    /// * example - 9
    forks_count: i64,
    /// * example - 80
    stargazers_count: i64,
    /// * example - 80
    watchers_count: i64,
    /// * example - 108
    size: i64,
    /// * example - master
    default_branch: String,
    ///
    open_issues_count: i64,
    /// * example - true
    is_template: bool,
    /// * example - octocat,atom,electron,API
    topics: Vec<String>,
    /// * example - true
    has_issues: bool,
    /// * example - true
    has_projects: bool,
    /// * example - true
    has_wiki: bool,
    ///
    has_pages: bool,
    /// * example - true
    has_downloads: bool,
    ///
    archived: bool,
    /// Returns whether or not this repository disabled.
    disabled: bool,
    /// * example - public
    visibility: String,
    /// * example - 2011-01-26T19:06:43Z
    pushed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:14:43Z
    updated_at: String,
    ///
    permissions: Value,
    /// * example - true
    allow_rebase_merge: bool,
    /// A git repository
    template_repository: Value,
    ///
    temp_clone_token: String,
    /// * example - true
    allow_squash_merge: bool,
    ///
    allow_auto_merge: bool,
    ///
    delete_branch_on_merge: bool,
    /// * example - true
    allow_merge_commit: bool,
    /// * example - true
    allow_forking: bool,
    /// * example - 42
    subscribers_count: i64,
    ///
    network_count: i64,
    /// License Simple
    license: Value,
    /// Simple User
    organization: Value,
    /// A git repository
    parent: Value,
    /// A git repository
    source: Value,
    ///
    forks: i64,
    ///
    master_branch: String,
    ///
    open_issues: i64,
    ///
    watchers: i64,
    /// Whether anonymous git access is allowed.
    anonymous_access_enabled: bool,
    /// Code of Conduct Simple
    code_of_conduct: Value,
    ///
    security_and_analysis: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsArtifactsResponse {
    ///
    total_count: i64,
    ///
    artifacts: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsArtifactsartifactIdResponse {
    /// * example - 5
    id: i64,
    /// * example - MDEwOkNoZWNrU3VpdGU1
    node_id: String,
    /// * example - AdventureWorks.Framework
    name: String,
    /// * example - 12345
    size_in_bytes: i64,
    /// * example - https://api.github.com/repos/github/hello-world/actions/artifacts/5
    url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/artifacts/5/zip
    archive_download_url: String,
    /// Whether or not the artifact has expired.
    expired: bool,
    ///
    created_at: String,
    ///
    expires_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsJobsjobIdResponse {
    /// * example - 21
    id: i64,
    /// * example - 5
    run_id: i64,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5
    run_url: String,
    /// * example - 1
    run_attempt: i64,
    /// * example - MDg6Q2hlY2tSdW40
    node_id: String,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/jobs/21
    url: String,
    /// * example - https://github.com/github/hello-world/runs/4
    html_url: String,
    /// * example - queued
    status: String,
    /// * example - success
    conclusion: String,
    /// * example - 2019-08-08T08:00:00-07:00
    started_at: String,
    /// * example - 2019-08-08T08:00:00-07:00
    completed_at: String,
    /// * example - test-coverage
    name: String,
    /// Steps in this job.
    steps: Vec<Value>,
    /// * example - https://api.github.com/repos/github/hello-world/check-runs/4
    check_run_url: String,
    /// * example - self-hosted,foo,bar
    labels: Vec<String>,
    /// * example - 1
    runner_id: i64,
    /// * example - my runner
    runner_name: String,
    /// * example - 2
    runner_group_id: i64,
    /// * example - my runner group
    runner_group_name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsPermissionsResponse {
    /// Whether GitHub Actions is enabled on the repository.
    enabled: bool,
    /// The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
    allowed_actions: String,
    /// The API URL to use to get or set the actions that are allowed to run, when `allowed_actions` is set to `selected`.
    selected_actions_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsPermissionsSelectedActionsResponse {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    github_owned_allowed: bool,
    /// Whether actions in GitHub Marketplace from verified creators are allowed. Set to `true` to allow all GitHub Marketplace actions by verified creators.
    verified_allowed: bool,
    /// Specifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/*`."
    patterns_allowed: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunnersResponse {
    ///
    total_count: i64,
    ///
    runners: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunnersrunnerIdResponse {
    /// * example - 5
    id: i64,
    /// * example - iMac
    name: String,
    /// * example - macos
    os: String,
    /// * example - online
    status: String,
    ///
    busy: bool,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoActionsRunnersrunnerIdLabelsResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoActionsRunnersrunnerIdLabelsnameResponse {
    ///
    total_count: i64,
    ///
    labels: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsResponse {
    ///
    total_count: i64,
    ///
    workflow_runs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdResponse {
    /// * example - 5
    id: i64,
    /// * example - Build
    name: String,
    /// * example - MDEwOkNoZWNrU3VpdGU1
    node_id: String,
    /// * example - 42
    check_suite_id: i64,
    /// * example - MDEwOkNoZWNrU3VpdGU0Mg==
    check_suite_node_id: String,
    /// * example - master
    head_branch: String,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - 106
    run_number: i64,
    /// * example - 1
    run_attempt: i64,
    /// * example - push
    event: String,
    /// * example - completed
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - 5
    workflow_id: i64,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5
    url: String,
    /// * example - https://github.com/github/hello-world/suites/4
    html_url: String,
    ///
    pull_requests: Vec<Value>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// The start time of the latest run. Resets on re-run.
    run_started_at: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/jobs
    jobs_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/logs
    logs_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-suites/12
    check_suite_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/rerun/artifacts
    artifacts_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/cancel
    cancel_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/rerun
    rerun_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/attempts/3
    previous_attempt_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/workflows/main.yaml
    workflow_url: String,
    /// Simple Commit
    head_commit: Value,
    /// Minimal Repository
    repository: Value,
    /// Minimal Repository
    head_repository: Value,
    /// * example - 5
    head_repository_id: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdArtifactsResponse {
    ///
    total_count: i64,
    ///
    artifacts: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberResponse {
    /// * example - 5
    id: i64,
    /// * example - Build
    name: String,
    /// * example - MDEwOkNoZWNrU3VpdGU1
    node_id: String,
    /// * example - 42
    check_suite_id: i64,
    /// * example - MDEwOkNoZWNrU3VpdGU0Mg==
    check_suite_node_id: String,
    /// * example - master
    head_branch: String,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - 106
    run_number: i64,
    /// * example - 1
    run_attempt: i64,
    /// * example - push
    event: String,
    /// * example - completed
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - 5
    workflow_id: i64,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5
    url: String,
    /// * example - https://github.com/github/hello-world/suites/4
    html_url: String,
    ///
    pull_requests: Vec<Value>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// The start time of the latest run. Resets on re-run.
    run_started_at: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/jobs
    jobs_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/logs
    logs_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-suites/12
    check_suite_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/rerun/artifacts
    artifacts_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/cancel
    cancel_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/rerun
    rerun_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/runs/5/attempts/3
    previous_attempt_url: String,
    /// * example - https://api.github.com/repos/github/hello-world/actions/workflows/main.yaml
    workflow_url: String,
    /// Simple Commit
    head_commit: Value,
    /// Minimal Repository
    repository: Value,
    /// Minimal Repository
    head_repository: Value,
    /// * example - 5
    head_repository_id: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobsResponse {
    ///
    total_count: i64,
    ///
    jobs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdJobsResponse {
    ///
    total_count: i64,
    ///
    jobs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsRunsrunIdTimingResponse {
    ///
    billable: Value,
    ///
    run_duration_ms: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsSecretsResponse {
    ///
    total_count: i64,
    ///
    secrets: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsSecretsPublicKeyResponse {
    /// * example - 1234567
    key_id: String,
    /// * example - hBT5WZEj8ZoOv6TYJsfWq7MxTEQopZO5/IT3ZCVQPzs=
    key: String,
    /// * example - 2
    id: i64,
    /// * example - https://api.github.com/user/keys/2
    url: String,
    /// * example - ssh-rsa AAAAB3NzaC1yc2EAAA
    title: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsSecretssecretNameResponse {
    /// * example - SECRET_TOKEN
    name: String,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsWorkflowsResponse {
    ///
    total_count: i64,
    ///
    workflows: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsWorkflowsworkflowIdResponse {
    /// * example - 5
    id: i64,
    /// * example - MDg6V29ya2Zsb3cxMg==
    node_id: String,
    /// * example - CI
    name: String,
    /// * example - ruby.yaml
    path: String,
    /// * example - active
    state: String,
    /// * example - 2019-12-06T14:20:20.000Z
    created_at: String,
    /// * example - 2019-12-06T14:20:20.000Z
    updated_at: String,
    /// * example - https://api.github.com/repos/actions/setup-ruby/workflows/5
    url: String,
    /// * example - https://github.com/actions/setup-ruby/blob/master/.github/workflows/ruby.yaml
    html_url: String,
    /// * example - https://github.com/actions/setup-ruby/workflows/CI/badge.svg
    badge_url: String,
    /// * example - 2019-12-06T14:20:20.000Z
    deleted_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsWorkflowsworkflowIdRunsResponse {
    ///
    total_count: i64,
    ///
    workflow_runs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoActionsWorkflowsworkflowIdTimingResponse {
    ///
    billable: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoAutolinksautolinkIdResponse {
    /// * example - 3
    id: i64,
    /// * example - TICKET-
    key_prefix: String,
    /// * example - https://example.com/TICKET?query=<num>
    url_template: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchResponse {
    ///
    name: String,
    /// Commit
    commit: Value,
    ///
    _links: Value,
    ///
    protected: bool,
    /// Branch Protection
    protection: Value,
    ///
    protection_url: String,
    /// * example - "mas*"
    pattern: String,
    /// * example - 1
    required_approving_review_count: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionResponse {
    ///
    url: String,
    ///
    enabled: bool,
    /// Protected Branch Required Status Check
    required_status_checks: Value,
    /// Protected Branch Admin Enforced
    enforce_admins: Value,
    /// Protected Branch Pull Request Review
    required_pull_request_reviews: Value,
    /// Branch Restriction Policy
    restrictions: Value,
    ///
    required_linear_history: Value,
    ///
    allow_force_pushes: Value,
    ///
    allow_deletions: Value,
    ///
    required_conversation_resolution: Value,
    /// * example - "branch/with/protection"
    name: String,
    /// * example - "https://api.github.com/repos/owner-79e94e2d36b3fd06a32bb213/AAA_Public_Repo/branches/branch/with/protection/protection"
    protection_url: String,
    ///
    required_signatures: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoBranchesbranchProtectionResponse {
    ///
    url: String,
    /// Status Check Policy
    required_status_checks: Value,
    ///
    required_pull_request_reviews: Value,
    ///
    required_signatures: Value,
    ///
    enforce_admins: Value,
    ///
    required_linear_history: Value,
    ///
    allow_force_pushes: Value,
    ///
    allow_deletions: Value,
    /// Branch Restriction Policy
    restrictions: Value,
    ///
    required_conversation_resolution: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionEnforceAdminsResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/enforce_admins
    url: String,
    /// * example - true
    enabled: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoBranchesbranchProtectionEnforceAdminsResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/enforce_admins
    url: String,
    /// * example - true
    enabled: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/dismissal_restrictions
    url: String,
    ///
    dismissal_restrictions: Value,
    /// * example - true
    dismiss_stale_reviews: bool,
    /// * example - true
    require_code_owner_reviews: bool,
    /// * example - 2
    required_approving_review_count: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/dismissal_restrictions
    url: String,
    ///
    dismissal_restrictions: Value,
    /// * example - true
    dismiss_stale_reviews: bool,
    /// * example - true
    require_code_owner_reviews: bool,
    /// * example - 2
    required_approving_review_count: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/enforce_admins
    url: String,
    /// * example - true
    enabled: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/enforce_admins
    url: String,
    /// * example - true
    enabled: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/required_status_checks
    url: String,
    /// * example - true
    strict: bool,
    /// * example - continuous-integration/travis-ci
    contexts: Vec<String>,
    ///
    checks: Vec<Value>,
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/required_status_checks/contexts
    contexts_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/required_status_checks
    url: String,
    /// * example - true
    strict: bool,
    /// * example - continuous-integration/travis-ci
    contexts: Vec<String>,
    ///
    checks: Vec<Value>,
    /// * example - https://api.github.com/repos/octocat/Hello-World/branches/master/protection/required_status_checks/contexts
    contexts_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoBranchesbranchProtectionRestrictionsResponse {
    ///
    url: String,
    ///
    users_url: String,
    ///
    teams_url: String,
    ///
    apps_url: String,
    ///
    users: Vec<Value>,
    ///
    teams: Vec<Value>,
    ///
    apps: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCheckRunscheckRunIdResponse {
    /// * example - 21
    id: i64,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - MDg6Q2hlY2tSdW40
    node_id: String,
    /// * example - 42
    external_id: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-runs/4
    url: String,
    /// * example - https://github.com/github/hello-world/runs/4
    html_url: String,
    /// * example - https://example.com
    details_url: String,
    /// * example - queued
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - 2018-05-04T01:14:52Z
    started_at: String,
    /// * example - 2018-05-04T01:14:52Z
    completed_at: String,
    ///
    output: Value,
    /// * example - test-coverage
    name: String,
    ///
    check_suite: Value,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    app: Value,
    ///
    pull_requests: Vec<Value>,
    /// A deployment created as the result of an Actions check run from a workflow that references an environment
    deployment: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoCheckRunscheckRunIdResponse {
    /// * example - 21
    id: i64,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - MDg6Q2hlY2tSdW40
    node_id: String,
    /// * example - 42
    external_id: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-runs/4
    url: String,
    /// * example - https://github.com/github/hello-world/runs/4
    html_url: String,
    /// * example - https://example.com
    details_url: String,
    /// * example - queued
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - 2018-05-04T01:14:52Z
    started_at: String,
    /// * example - 2018-05-04T01:14:52Z
    completed_at: String,
    ///
    output: Value,
    /// * example - test-coverage
    name: String,
    ///
    check_suite: Value,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    app: Value,
    ///
    pull_requests: Vec<Value>,
    /// A deployment created as the result of an Actions check run from a workflow that references an environment
    deployment: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoCheckSuitesResponse {
    /// * example - 5
    id: i64,
    /// * example - MDEwOkNoZWNrU3VpdGU1
    node_id: String,
    /// * example - master
    head_branch: String,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - completed
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-suites/5
    url: String,
    /// * example - 146e867f55c26428e5f9fade55a9bbf5e95a7912
    before: String,
    /// * example - d6fde92930d4715a2b49857d24b940956b26d2d3
    after: String,
    ///
    pull_requests: Vec<Value>,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    app: Value,
    /// Minimal Repository
    repository: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Simple Commit
    head_commit: Value,
    ///
    latest_check_runs_count: i64,
    ///
    check_runs_url: String,
    ///
    rerequestable: bool,
    ///
    runs_rerequestable: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoCheckSuitesPreferencesResponse {
    ///
    preferences: Value,
    /// Minimal Repository
    repository: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCheckSuitescheckSuiteIdResponse {
    /// * example - 5
    id: i64,
    /// * example - MDEwOkNoZWNrU3VpdGU1
    node_id: String,
    /// * example - master
    head_branch: String,
    /// * example - 009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d
    head_sha: String,
    /// * example - completed
    status: String,
    /// * example - neutral
    conclusion: String,
    /// * example - https://api.github.com/repos/github/hello-world/check-suites/5
    url: String,
    /// * example - 146e867f55c26428e5f9fade55a9bbf5e95a7912
    before: String,
    /// * example - d6fde92930d4715a2b49857d24b940956b26d2d3
    after: String,
    ///
    pull_requests: Vec<Value>,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    app: Value,
    /// Minimal Repository
    repository: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Simple Commit
    head_commit: Value,
    ///
    latest_check_runs_count: i64,
    ///
    check_runs_url: String,
    ///
    rerequestable: bool,
    ///
    runs_rerequestable: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCheckSuitescheckSuiteIdCheckRunsResponse {
    ///
    total_count: i64,
    ///
    check_runs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCodeScanningAlertsalertNumberResponse {
    /// The security alert number.
    number: i64,
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    created_at: String,
    /// The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    updated_at: String,
    /// The REST API URL of the alert resource.
    url: String,
    /// The GitHub URL of the alert resource.
    html_url: String,
    /// The REST API URL for fetching the list of instances for an alert.
    instances_url: String,
    /// State of a code scanning alert.
    state: String,
    /// The time that the alert was no longer detected and was considered fixed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    fixed_at: String,
    /// Simple User
    dismissed_by: Value,
    /// The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    dismissed_at: String,
    /// **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
    dismissed_reason: String,
    ///
    rule: Value,
    ///
    tool: Value,
    ///
    most_recent_instance: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoCodeScanningAlertsalertNumberResponse {
    /// The security alert number.
    number: i64,
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    created_at: String,
    /// The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    updated_at: String,
    /// The REST API URL of the alert resource.
    url: String,
    /// The GitHub URL of the alert resource.
    html_url: String,
    /// The REST API URL for fetching the list of instances for an alert.
    instances_url: String,
    /// State of a code scanning alert.
    state: String,
    /// The time that the alert was no longer detected and was considered fixed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    fixed_at: String,
    /// Simple User
    dismissed_by: Value,
    /// The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    dismissed_at: String,
    /// **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
    dismissed_reason: String,
    ///
    rule: Value,
    ///
    tool: Value,
    ///
    most_recent_instance: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCodeScanningAnalysesanalysisIdResponse {
    /// The full Git reference, formatted as `refs/heads/<branch name>`,
    /// `refs/pull/<number>/merge`, or `refs/pull/<number>/head`.
    #[serde(rename = "ref")]
    aref: String,
    /// The SHA of the commit to which the analysis you are uploading relates.
    commit_sha: String,
    /// Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name.
    analysis_key: String,
    /// Identifies the variable values associated with the environment in which this analysis was performed.
    environment: String,
    /// Identifies the configuration under which the analysis was executed. Used to distinguish between multiple analyses for the same tool and commit, but performed on different languages or different parts of the code.
    category: String,
    /// * example - error reading field xyz
    error: String,
    /// The time that the analysis was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    created_at: String,
    /// The total number of results in the analysis.
    results_count: i64,
    /// The total number of rules used in the analysis.
    rules_count: i64,
    /// Unique identifier for this analysis.
    id: i64,
    /// The REST API URL of the analysis resource.
    url: String,
    /// * example - 6c81cd8e-b078-4ac3-a3be-1dad7dbd0b53
    sarif_id: String,
    ///
    tool: Value,
    ///
    deletable: bool,
    /// * example - 123 results were ignored
    warning: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoCodeScanningAnalysesanalysisIdResponse {
    /// Next deletable analysis in chain, without last analysis deletion confirmation
    next_analysis_url: String,
    /// Next deletable analysis in chain, with last analysis deletion confirmation
    confirm_delete_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCodeScanningSarifssarifIdResponse {
    /// `pending` files have not yet been processed, while `complete` means results from the SARIF have been stored. `failed` files have either not been processed at all, or could only be partially processed.
    processing_status: String,
    /// The REST API URL for getting the analyses associated with the upload.
    analyses_url: String,
    /// Any errors that ocurred during processing of the delivery.
    errors: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCodespacesResponse {
    ///
    total_count: i64,
    ///
    codespaces: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCodespacesMachinesResponse {
    ///
    total_count: i64,
    ///
    machines: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCollaboratorsusernamePermissionResponse {
    ///
    permission: String,
    /// * example - admin
    role_name: String,
    /// Collaborator
    user: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommentscommentIdResponse {
    ///
    html_url: String,
    ///
    url: String,
    ///
    id: i64,
    ///
    node_id: String,
    ///
    body: String,
    ///
    path: String,
    ///
    position: i64,
    ///
    line: i64,
    ///
    commit_id: String,
    /// Simple User
    user: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - OWNER
    author_association: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoCommentscommentIdResponse {
    ///
    html_url: String,
    ///
    url: String,
    ///
    id: i64,
    ///
    node_id: String,
    ///
    body: String,
    ///
    path: String,
    ///
    position: i64,
    ///
    line: i64,
    ///
    commit_id: String,
    /// Simple User
    user: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - OWNER
    author_association: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoCommentscommentIdReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommitsrefResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e
    url: String,
    /// * example - 6dcb09b5b57875f334f61aebed695e2e4193db5e
    sha: String,
    /// * example - MDY6Q29tbWl0NmRjYjA5YjViNTc4NzVmMzM0ZjYxYWViZWQ2OTVlMmU0MTkzZGI1ZQ==
    node_id: String,
    /// * example - https://github.com/octocat/Hello-World/commit/6dcb09b5b57875f334f61aebed695e2e4193db5e
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e/comments
    comments_url: String,
    ///
    commit: Value,
    /// Simple User
    author: Value,
    /// Simple User
    committer: Value,
    ///
    parents: Vec<Value>,
    ///
    stats: Value,
    ///
    files: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommitsrefCheckRunsResponse {
    ///
    total_count: i64,
    ///
    check_runs: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommitsrefCheckSuitesResponse {
    ///
    total_count: i64,
    ///
    check_suites: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommitsrefStatusResponse {
    ///
    state: String,
    ///
    statuses: Vec<Value>,
    ///
    sha: String,
    ///
    total_count: i64,
    /// Minimal Repository
    repository: Value,
    ///
    commit_url: String,
    ///
    url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoCommunityProfileResponse {
    /// * example - 100
    health_percentage: i64,
    /// * example - My first repository on GitHub!
    description: String,
    /// * example - example.com
    documentation: String,
    ///
    files: Value,
    /// * example - 2017-02-28T19:09:29Z
    updated_at: String,
    /// * example - true
    content_reports_enabled: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoComparebaseheadResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/compare/master...topic
    url: String,
    /// * example - https://github.com/octocat/Hello-World/compare/master...topic
    html_url: String,
    /// * example - https://github.com/octocat/Hello-World/compare/octocat:bbcd538c8e72b8c175046e27cc8f907076331401...octocat:0328041d1152db8ae77652d1618a02e57f745f17
    permalink_url: String,
    /// * example - https://github.com/octocat/Hello-World/compare/master...topic.diff
    diff_url: String,
    /// * example - https://github.com/octocat/Hello-World/compare/master...topic.patch
    patch_url: String,
    /// Commit
    base_commit: Value,
    /// Commit
    merge_base_commit: Value,
    /// * example - ahead
    status: String,
    /// * example - 4
    ahead_by: i64,
    /// * example - 5
    behind_by: i64,
    /// * example - 6
    total_commits: i64,
    ///
    commits: Vec<Value>,
    ///
    files: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoContentspathResponse {
    ///
    content: Value,
    ///
    commit: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoContentspathResponse {
    ///
    content: Value,
    ///
    commit: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoDeploymentsdeploymentIdResponse {
    /// * example - https://api.github.com/repos/octocat/example/deployments/1
    url: String,
    /// * example - 42
    id: i64,
    /// * example - MDEwOkRlcGxveW1lbnQx
    node_id: String,
    /// * example - a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d
    sha: String,
    /// * example - topic-branch
    #[serde(rename = "ref")]
    aref: String,
    /// * example - deploy
    task: String,
    /// * example - staging
    original_environment: String,
    /// * example - production
    environment: String,
    /// * example - Deploy request from hubot
    description: String,
    /// Simple User
    creator: Value,
    /// * example - 2012-07-20T01:19:13Z
    created_at: String,
    /// * example - 2012-07-20T01:19:13Z
    updated_at: String,
    /// * example - https://api.github.com/repos/octocat/example/deployments/1/statuses
    statuses_url: String,
    /// * example - https://api.github.com/repos/octocat/example
    repository_url: String,
    /// * example - true
    transient_environment: bool,
    /// * example - true
    production_environment: bool,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoDeploymentsdeploymentIdStatusesstatusIdResponse {
    /// * example - https://api.github.com/repos/octocat/example/deployments/42/statuses/1
    url: String,
    /// * example - 1
    id: i64,
    /// * example - MDE2OkRlcGxveW1lbnRTdGF0dXMx
    node_id: String,
    /// * example - success
    state: String,
    /// Simple User
    creator: Value,
    /// * example - Deployment finished successfully.
    description: String,
    /// * example - production
    environment: String,
    /// * example - https://example.com/deployment/42/output
    target_url: String,
    /// * example - 2012-07-20T01:19:13Z
    created_at: String,
    /// * example - 2012-07-20T01:19:13Z
    updated_at: String,
    /// * example - https://api.github.com/repos/octocat/example/deployments/42
    deployment_url: String,
    /// * example - https://api.github.com/repos/octocat/example
    repository_url: String,
    /// * example - https://staging.example.com/
    environment_url: String,
    /// * example - https://example.com/deployment/42/output
    log_url: String,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoEnvironmentsResponse {
    /// * example - 5
    total_count: i64,
    ///
    environments: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoEnvironmentsenvironmentNameResponse {
    /// * example - 56780428
    id: i64,
    /// * example - MDExOkVudmlyb25tZW50NTY3ODA0Mjg=
    node_id: String,
    /// * example - staging
    name: String,
    /// * example - https://api.github.com/repos/github/hello-world/environments/staging
    url: String,
    /// * example - https://github.com/github/hello-world/deployments/activity_log?environments_filter=staging
    html_url: String,
    /// * example - 2020-11-23T22:00:40Z
    created_at: String,
    /// * example - 2020-11-23T22:00:40Z
    updated_at: String,
    /// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
    deployment_branch_policy: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoEnvironmentsenvironmentNameResponse {
    /// * example - 56780428
    id: i64,
    /// * example - MDExOkVudmlyb25tZW50NTY3ODA0Mjg=
    node_id: String,
    /// * example - staging
    name: String,
    /// * example - https://api.github.com/repos/github/hello-world/environments/staging
    url: String,
    /// * example - https://github.com/github/hello-world/deployments/activity_log?environments_filter=staging
    html_url: String,
    /// * example - 2020-11-23T22:00:40Z
    created_at: String,
    /// * example - 2020-11-23T22:00:40Z
    updated_at: String,
    /// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
    deployment_branch_policy: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoGitBlobsfileShaResponse {
    ///
    content: String,
    ///
    encoding: String,
    ///
    url: String,
    ///
    sha: String,
    ///
    size: i64,
    ///
    node_id: String,
    ///
    highlighted_content: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoGitCommitscommitShaResponse {
    /// * example - 7638417db6d59f3c431d3e1f261cc637155684cd
    sha: String,
    ///
    node_id: String,
    ///
    url: String,
    /// Identifying information for the git-user
    author: Value,
    /// Identifying information for the git-user
    committer: Value,
    /// * example - Fix #42
    message: String,
    ///
    tree: Value,
    ///
    parents: Vec<Value>,
    ///
    verification: Value,
    ///
    html_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoGitRefrefResponse {
    ///
    #[serde(rename = "ref")]
    aref: String,
    ///
    node_id: String,
    ///
    url: String,
    ///
    object: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoGitRefsrefResponse {
    ///
    #[serde(rename = "ref")]
    aref: String,
    ///
    node_id: String,
    ///
    url: String,
    ///
    object: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoGitTagstagShaResponse {
    /// * example - MDM6VGFnOTQwYmQzMzYyNDhlZmFlMGY5ZWU1YmM3YjJkNWM5ODU4ODdiMTZhYw==
    node_id: String,
    /// * example - v0.0.1
    tag: String,
    /// * example - 940bd336248efae0f9ee5bc7b2d5c985887b16ac
    sha: String,
    /// * example - https://api.github.com/repositories/42/git/tags/940bd336248efae0f9ee5bc7b2d5c985887b16ac
    url: String,
    /// * example - Initial public release
    message: String,
    ///
    tagger: Value,
    ///
    object: Value,
    ///
    verification: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoGitTreestreeShaResponse {
    ///
    sha: String,
    ///
    url: String,
    ///
    truncated: bool,
    /// * example - [object Object]
    tree: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoHookshookIdResponse {
    ///
    #[serde(rename = "type")]
    atype: String,
    /// * example - 42
    id: i64,
    /// * example - web
    name: String,
    /// * example - true
    active: bool,
    /// * example - push,pull_request
    events: Vec<String>,
    ///
    config: Value,
    /// * example - 2011-09-06T20:39:23Z
    updated_at: String,
    /// * example - 2011-09-06T17:26:27Z
    created_at: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1
    url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/test
    test_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/pings
    ping_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/deliveries
    deliveries_url: String,
    ///
    last_response: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoHookshookIdResponse {
    ///
    #[serde(rename = "type")]
    atype: String,
    /// * example - 42
    id: i64,
    /// * example - web
    name: String,
    /// * example - true
    active: bool,
    /// * example - push,pull_request
    events: Vec<String>,
    ///
    config: Value,
    /// * example - 2011-09-06T20:39:23Z
    updated_at: String,
    /// * example - 2011-09-06T17:26:27Z
    created_at: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1
    url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/test
    test_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/pings
    ping_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/hooks/1/deliveries
    deliveries_url: String,
    ///
    last_response: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoHookshookIdConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoHookshookIdConfigResponse {
    /// * example - https://example.com/webhook
    url: String,
    /// * example - "json"
    content_type: String,
    /// * example - "********"
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoHookshookIdDeliveriesdeliveryIdResponse {
    /// * example - 42
    id: i64,
    /// * example - 58474f00-b361-11eb-836d-0e4f3503ccbe
    guid: String,
    /// * example - 2021-05-12T20:33:44Z
    delivered_at: String,
    /// Whether the delivery is a redelivery.
    redelivery: bool,
    /// * example - 0.03
    duration: f64,
    /// * example - failed to connect
    status: String,
    /// * example - 502
    status_code: i64,
    /// * example - issues
    event: String,
    /// * example - opened
    action: String,
    /// * example - 123
    installation_id: i64,
    /// * example - 123
    repository_id: i64,
    /// * example - https://www.example.com
    url: String,
    ///
    request: Value,
    ///
    response: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoImportResponse {
    ///
    vcs: String,
    ///
    use_lfs: bool,
    /// The URL of the originating repository.
    vcs_url: String,
    ///
    svc_root: String,
    ///
    tfvc_project: String,
    ///
    status: String,
    ///
    status_text: String,
    ///
    failed_step: String,
    ///
    error_message: String,
    ///
    import_percent: i64,
    ///
    commit_count: i64,
    ///
    push_percent: i64,
    ///
    has_large_files: bool,
    ///
    large_files_size: i64,
    ///
    large_files_count: i64,
    ///
    project_choices: Vec<Value>,
    ///
    message: String,
    ///
    authors_count: i64,
    ///
    url: String,
    ///
    html_url: String,
    ///
    authors_url: String,
    ///
    repository_url: String,
    ///
    svn_root: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoImportResponse {
    ///
    vcs: String,
    ///
    use_lfs: bool,
    /// The URL of the originating repository.
    vcs_url: String,
    ///
    svc_root: String,
    ///
    tfvc_project: String,
    ///
    status: String,
    ///
    status_text: String,
    ///
    failed_step: String,
    ///
    error_message: String,
    ///
    import_percent: i64,
    ///
    commit_count: i64,
    ///
    push_percent: i64,
    ///
    has_large_files: bool,
    ///
    large_files_size: i64,
    ///
    large_files_count: i64,
    ///
    project_choices: Vec<Value>,
    ///
    message: String,
    ///
    authors_count: i64,
    ///
    url: String,
    ///
    html_url: String,
    ///
    authors_url: String,
    ///
    repository_url: String,
    ///
    svn_root: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoImportAuthorsauthorIdResponse {
    ///
    id: i64,
    ///
    remote_id: String,
    ///
    remote_name: String,
    ///
    email: String,
    ///
    name: String,
    ///
    url: String,
    ///
    import_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoImportLfsResponse {
    ///
    vcs: String,
    ///
    use_lfs: bool,
    /// The URL of the originating repository.
    vcs_url: String,
    ///
    svc_root: String,
    ///
    tfvc_project: String,
    ///
    status: String,
    ///
    status_text: String,
    ///
    failed_step: String,
    ///
    error_message: String,
    ///
    import_percent: i64,
    ///
    commit_count: i64,
    ///
    push_percent: i64,
    ///
    has_large_files: bool,
    ///
    large_files_size: i64,
    ///
    large_files_count: i64,
    ///
    project_choices: Vec<Value>,
    ///
    message: String,
    ///
    authors_count: i64,
    ///
    url: String,
    ///
    html_url: String,
    ///
    authors_url: String,
    ///
    repository_url: String,
    ///
    svn_root: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoInstallationResponse {
    /// * example - 1
    id: i64,
    /// Describe whether all repositories have been selected or there's a selection involved
    repository_selection: String,
    /// * example - https://api.github.com/installations/1/access_tokens
    access_tokens_url: String,
    /// * example - https://api.github.com/installation/repositories
    repositories_url: String,
    /// * example - https://github.com/organizations/github/settings/installations/1
    html_url: String,
    /// * example - 1
    app_id: i64,
    /// The ID of the user or organization this token is being scoped to.
    target_id: i64,
    /// * example - Organization
    target_type: String,
    /// * example - [object Object]
    permissions: Value,
    ///
    events: Vec<String>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - config.yaml
    single_file_name: String,
    /// * example - true
    has_multiple_single_files: bool,
    /// * example - config.yml,.github/issue_TEMPLATE.md
    single_file_paths: Vec<String>,
    /// * example - github-actions
    app_slug: String,
    /// Simple User
    suspended_by: Value,
    ///
    suspended_at: String,
    /// * example - "test_13f1e99741e3e004@d7e1eb0bc0a1ba12.com"
    contact_email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoInteractionLimitsResponse {
    /// * example - collaborators_only
    limit: String,
    /// * example - repository
    origin: String,
    /// * example - 2018-08-17T04:18:39Z
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoInvitationsinvitationIdResponse {
    /// * example - 42
    id: i64,
    /// Minimal Repository
    repository: Value,
    /// Simple User
    invitee: Value,
    /// Simple User
    inviter: Value,
    /// * example - read
    permissions: String,
    /// * example - 2016-06-13T14:52:50-05:00
    created_at: String,
    /// Whether or not the invitation has expired
    expired: bool,
    /// * example - https://api.github.com/user/repository-invitations/1
    url: String,
    /// * example - https://github.com/octocat/Hello-World/invitations
    html_url: String,
    ///
    node_id: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoIssuesCommentscommentIdResponse {
    /// * example - 42
    id: i64,
    ///
    node_id: String,
    /// * example - https://api.github.com/repositories/42/issues/comments/1
    url: String,
    /// * example - What version of Safari were you using when you observed this bug?
    body: String,
    ///
    body_text: String,
    ///
    body_html: String,
    ///
    html_url: String,
    /// Simple User
    user: Value,
    /// * example - 2011-04-14T16:00:49Z
    created_at: String,
    /// * example - 2011-04-14T16:00:49Z
    updated_at: String,
    ///
    issue_url: String,
    /// * example - OWNER
    author_association: String,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoIssuesCommentscommentIdResponse {
    /// * example - 42
    id: i64,
    ///
    node_id: String,
    /// * example - https://api.github.com/repositories/42/issues/comments/1
    url: String,
    /// * example - What version of Safari were you using when you observed this bug?
    body: String,
    ///
    body_text: String,
    ///
    body_html: String,
    ///
    html_url: String,
    /// Simple User
    user: Value,
    /// * example - 2011-04-14T16:00:49Z
    created_at: String,
    /// * example - 2011-04-14T16:00:49Z
    updated_at: String,
    ///
    issue_url: String,
    /// * example - OWNER
    author_association: String,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoIssuesCommentscommentIdReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoIssuesEventseventIdResponse {
    /// * example - 1
    id: i64,
    /// * example - MDEwOklzc3VlRXZlbnQx
    node_id: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/events/1
    url: String,
    /// Simple User
    actor: Value,
    /// * example - closed
    event: String,
    /// * example - 6dcb09b5b57875f334f61aebed695e2e4193db5e
    commit_id: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/commits/6dcb09b5b57875f334f61aebed695e2e4193db5e
    commit_url: String,
    /// * example - 2011-04-14T16:00:49Z
    created_at: String,
    /// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
    issue: Value,
    /// Issue Event Label
    label: Value,
    /// Simple User
    assignee: Value,
    /// Simple User
    assigner: Value,
    /// Simple User
    review_requester: Value,
    /// Simple User
    requested_reviewer: Value,
    /// Groups of organization members that gives permissions on specified repositories.
    requested_team: Value,
    ///
    dismissed_review: Value,
    /// Issue Event Milestone
    milestone: Value,
    /// Issue Event Project Card
    project_card: Value,
    /// Issue Event Rename
    rename: Value,
    /// * example - OWNER
    author_association: String,
    ///
    lock_reason: String,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoIssuesissueNumberResponse {
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - https://api.github.com/repositories/42/issues/1
    url: String,
    ///
    repository_url: String,
    ///
    labels_url: String,
    ///
    comments_url: String,
    ///
    events_url: String,
    ///
    html_url: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - Widget creation fails in Safari on OS X 10.8
    title: String,
    /// * example - It looks like the new widget form is broken on Safari. When I try and create the widget, Safari crashes. This is reproducible on 10.8, but not 10.9. Maybe a browser bug?
    body: String,
    /// Simple User
    user: Value,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    ///
    locked: bool,
    ///
    active_lock_reason: String,
    ///
    comments: i64,
    ///
    pull_request: Value,
    ///
    closed_at: String,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    draft: bool,
    /// Simple User
    closed_by: Value,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    timeline_url: String,
    /// A git repository
    repository: Value,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
    /// * example - OWNER
    author_association: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoIssuesissueNumberResponse {
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - https://api.github.com/repositories/42/issues/1
    url: String,
    ///
    repository_url: String,
    ///
    labels_url: String,
    ///
    comments_url: String,
    ///
    events_url: String,
    ///
    html_url: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - Widget creation fails in Safari on OS X 10.8
    title: String,
    /// * example - It looks like the new widget form is broken on Safari. When I try and create the widget, Safari crashes. This is reproducible on 10.8, but not 10.9. Maybe a browser bug?
    body: String,
    /// Simple User
    user: Value,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    ///
    locked: bool,
    ///
    active_lock_reason: String,
    ///
    comments: i64,
    ///
    pull_request: Value,
    ///
    closed_at: String,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    draft: bool,
    /// Simple User
    closed_by: Value,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    timeline_url: String,
    /// A git repository
    repository: Value,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
    /// * example - OWNER
    author_association: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoIssuesissueNumberAssigneesResponse {
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - https://api.github.com/repositories/42/issues/1
    url: String,
    ///
    repository_url: String,
    ///
    labels_url: String,
    ///
    comments_url: String,
    ///
    events_url: String,
    ///
    html_url: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - Widget creation fails in Safari on OS X 10.8
    title: String,
    /// * example - It looks like the new widget form is broken on Safari. When I try and create the widget, Safari crashes. This is reproducible on 10.8, but not 10.9. Maybe a browser bug?
    body: String,
    /// Simple User
    user: Value,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    ///
    locked: bool,
    ///
    active_lock_reason: String,
    ///
    comments: i64,
    ///
    pull_request: Value,
    ///
    closed_at: String,
    ///
    created_at: String,
    ///
    updated_at: String,
    ///
    draft: bool,
    /// Simple User
    closed_by: Value,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    timeline_url: String,
    /// A git repository
    repository: Value,
    /// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
    performed_via_github_app: Value,
    /// * example - OWNER
    author_association: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoIssuesissueNumberReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoKeyskeyIdResponse {
    ///
    id: i64,
    ///
    key: String,
    ///
    url: String,
    ///
    title: String,
    ///
    verified: bool,
    ///
    created_at: String,
    ///
    read_only: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoLabelsnameResponse {
    /// * example - 208045946
    id: i64,
    /// * example - MDU6TGFiZWwyMDgwNDU5NDY=
    node_id: String,
    /// * example - https://api.github.com/repositories/42/labels/bug
    url: String,
    /// * example - bug
    name: String,
    /// * example - Something isn't working
    description: String,
    /// * example - FFFFFF
    color: String,
    /// * example - true
    default: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoLabelsnameResponse {
    /// * example - 208045946
    id: i64,
    /// * example - MDU6TGFiZWwyMDgwNDU5NDY=
    node_id: String,
    /// * example - https://api.github.com/repositories/42/labels/bug
    url: String,
    /// * example - bug
    name: String,
    /// * example - Something isn't working
    description: String,
    /// * example - FFFFFF
    color: String,
    /// * example - true
    default: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoLicenseResponse {
    ///
    name: String,
    ///
    path: String,
    ///
    sha: String,
    ///
    size: i64,
    ///
    url: String,
    ///
    html_url: String,
    ///
    git_url: String,
    ///
    download_url: String,
    ///
    #[serde(rename = "type")]
    atype: String,
    ///
    content: String,
    ///
    encoding: String,
    ///
    _links: Value,
    /// License Simple
    license: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoMergeUpstreamResponse {
    ///
    message: String,
    ///
    merge_type: String,
    ///
    base_branch: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoMilestonesmilestoneNumberResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/milestones/1
    url: String,
    /// * example - https://github.com/octocat/Hello-World/milestones/v1.0
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/milestones/1/labels
    labels_url: String,
    /// * example - 1002604
    id: i64,
    /// * example - MDk6TWlsZXN0b25lMTAwMjYwNA==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - v1.0
    title: String,
    /// * example - Tracking milestone for version 1.0
    description: String,
    /// Simple User
    creator: Value,
    /// * example - 4
    open_issues: i64,
    /// * example - 8
    closed_issues: i64,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// * example - 2013-02-12T13:22:01Z
    closed_at: String,
    /// * example - 2012-10-09T23:39:01Z
    due_on: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoMilestonesmilestoneNumberResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/milestones/1
    url: String,
    /// * example - https://github.com/octocat/Hello-World/milestones/v1.0
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/milestones/1/labels
    labels_url: String,
    /// * example - 1002604
    id: i64,
    /// * example - MDk6TWlsZXN0b25lMTAwMjYwNA==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - v1.0
    title: String,
    /// * example - Tracking milestone for version 1.0
    description: String,
    /// Simple User
    creator: Value,
    /// * example - 4
    open_issues: i64,
    /// * example - 8
    closed_issues: i64,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// * example - 2013-02-12T13:22:01Z
    closed_at: String,
    /// * example - 2012-10-09T23:39:01Z
    due_on: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPagesResponse {
    /// * example - https://api.github.com/repos/github/hello-world/pages
    url: String,
    /// * example - built
    status: String,
    /// * example - example.com
    cname: String,
    /// * example - pending
    protected_domain_state: String,
    /// The timestamp when a pending domain becomes unverified.
    pending_domain_unverified_at: String,
    /// Whether the Page has a custom 404 page.
    custom_404: bool,
    /// * example - https://example.com
    html_url: String,
    ///
    source: Value,
    /// * example - true
    public: bool,
    ///
    https_certificate: Value,
    /// * example - true
    https_enforced: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPagesBuildsLatestResponse {
    ///
    url: String,
    ///
    status: String,
    ///
    error: Value,
    /// Simple User
    pusher: Value,
    ///
    commit: String,
    ///
    duration: i64,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPagesBuildsbuildIdResponse {
    ///
    url: String,
    ///
    status: String,
    ///
    error: Value,
    /// Simple User
    pusher: Value,
    ///
    commit: String,
    ///
    duration: i64,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPagesHealthResponse {
    ///
    domain: Value,
    ///
    alt_domain: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPullsCommentscommentIdResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/comments/1
    url: String,
    /// * example - 42
    pull_request_review_id: i64,
    /// * example - 1
    id: i64,
    /// * example - MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDEw
    node_id: String,
    /// * example - @@ -16,33 +16,40 @@ public class Connection : IConnection...
    diff_hunk: String,
    /// * example - config/database.yaml
    path: String,
    /// * example - 1
    position: i64,
    /// * example - 4
    original_position: i64,
    /// * example - 6dcb09b5b57875f334f61aebed695e2e4193db5e
    commit_id: String,
    /// * example - 9c48853fa3dc5c1c3d6f1f1cd1f2743e72652840
    original_commit_id: String,
    /// * example - 8
    in_reply_to_id: i64,
    /// Simple User
    user: Value,
    /// * example - We should probably include a check for null values here.
    body: String,
    /// * example - 2011-04-14T16:00:49Z
    created_at: String,
    /// * example - 2011-04-14T16:00:49Z
    updated_at: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1#discussion-diff-1
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1
    pull_request_url: String,
    /// * example - OWNER
    author_association: String,
    ///
    _links: Value,
    /// * example - 2
    start_line: i64,
    /// * example - 2
    original_start_line: i64,
    /// The side of the first line of the range for a multi-line comment.
    start_side: String,
    /// * example - 2
    line: i64,
    /// * example - 2
    original_line: i64,
    /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
    side: String,
    ///
    reactions: Value,
    /// * example - "<p>comment body</p>"
    body_html: String,
    /// * example - "comment body"
    body_text: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoPullsCommentscommentIdResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/comments/1
    url: String,
    /// * example - 42
    pull_request_review_id: i64,
    /// * example - 1
    id: i64,
    /// * example - MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDEw
    node_id: String,
    /// * example - @@ -16,33 +16,40 @@ public class Connection : IConnection...
    diff_hunk: String,
    /// * example - config/database.yaml
    path: String,
    /// * example - 1
    position: i64,
    /// * example - 4
    original_position: i64,
    /// * example - 6dcb09b5b57875f334f61aebed695e2e4193db5e
    commit_id: String,
    /// * example - 9c48853fa3dc5c1c3d6f1f1cd1f2743e72652840
    original_commit_id: String,
    /// * example - 8
    in_reply_to_id: i64,
    /// Simple User
    user: Value,
    /// * example - We should probably include a check for null values here.
    body: String,
    /// * example - 2011-04-14T16:00:49Z
    created_at: String,
    /// * example - 2011-04-14T16:00:49Z
    updated_at: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1#discussion-diff-1
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1
    pull_request_url: String,
    /// * example - OWNER
    author_association: String,
    ///
    _links: Value,
    /// * example - 2
    start_line: i64,
    /// * example - 2
    original_start_line: i64,
    /// The side of the first line of the range for a multi-line comment.
    start_side: String,
    /// * example - 2
    line: i64,
    /// * example - 2
    original_line: i64,
    /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
    side: String,
    ///
    reactions: Value,
    /// * example - "<p>comment body</p>"
    body_html: String,
    /// * example - "comment body"
    body_text: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoPullsCommentscommentIdReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPullspullNumberResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347
    url: String,
    /// * example - 1
    id: i64,
    /// * example - MDExOlB1bGxSZXF1ZXN0MQ==
    node_id: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347
    html_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.diff
    diff_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.patch
    patch_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347
    issue_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/commits
    commits_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/comments
    review_comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/comments{/number}
    review_comment_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347/comments
    comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/statuses/6dcb09b5b57875f334f61aebed695e2e4193db5e
    statuses_url: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - true
    locked: bool,
    /// * example - Amazing new feature
    title: String,
    /// Simple User
    user: Value,
    /// * example - Please pull these awesome changes
    body: String,
    ///
    labels: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    /// * example - too heated
    active_lock_reason: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    closed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    merged_at: String,
    /// * example - e5bd3914e2e596debea16f433f57875b5b90bcd6
    merge_commit_sha: String,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    ///
    requested_reviewers: Vec<Value>,
    ///
    requested_teams: Vec<Value>,
    ///
    head: Value,
    ///
    base: Value,
    ///
    _links: Value,
    /// * example - OWNER
    author_association: String,
    /// The status of auto merging a pull request.
    auto_merge: Value,
    /// Indicates whether or not the pull request is a draft.
    draft: bool,
    ///
    merged: bool,
    /// * example - true
    mergeable: bool,
    /// * example - true
    rebaseable: bool,
    /// * example - clean
    mergeable_state: String,
    /// Simple User
    merged_by: Value,
    /// * example - 10
    comments: i64,
    ///
    review_comments: i64,
    /// * example - true
    maintainer_can_modify: bool,
    /// * example - 3
    commits: i64,
    /// * example - 100
    additions: i64,
    /// * example - 3
    deletions: i64,
    /// * example - 5
    changed_files: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoPullspullNumberResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347
    url: String,
    /// * example - 1
    id: i64,
    /// * example - MDExOlB1bGxSZXF1ZXN0MQ==
    node_id: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347
    html_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.diff
    diff_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.patch
    patch_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347
    issue_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/commits
    commits_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/comments
    review_comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/comments{/number}
    review_comment_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347/comments
    comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/statuses/6dcb09b5b57875f334f61aebed695e2e4193db5e
    statuses_url: String,
    /// * example - 42
    number: i64,
    /// * example - open
    state: String,
    /// * example - true
    locked: bool,
    /// * example - Amazing new feature
    title: String,
    /// Simple User
    user: Value,
    /// * example - Please pull these awesome changes
    body: String,
    ///
    labels: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    /// * example - too heated
    active_lock_reason: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    closed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    merged_at: String,
    /// * example - e5bd3914e2e596debea16f433f57875b5b90bcd6
    merge_commit_sha: String,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    ///
    requested_reviewers: Vec<Value>,
    ///
    requested_teams: Vec<Value>,
    ///
    head: Value,
    ///
    base: Value,
    ///
    _links: Value,
    /// * example - OWNER
    author_association: String,
    /// The status of auto merging a pull request.
    auto_merge: Value,
    /// Indicates whether or not the pull request is a draft.
    draft: bool,
    ///
    merged: bool,
    /// * example - true
    mergeable: bool,
    /// * example - true
    rebaseable: bool,
    /// * example - clean
    mergeable_state: String,
    /// Simple User
    merged_by: Value,
    /// * example - 10
    comments: i64,
    ///
    review_comments: i64,
    /// * example - true
    maintainer_can_modify: bool,
    /// * example - 3
    commits: i64,
    /// * example - 100
    additions: i64,
    /// * example - 3
    deletions: i64,
    /// * example - 5
    changed_files: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoPullspullNumberMergeResponse {
    ///
    sha: String,
    ///
    merged: bool,
    ///
    message: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPullspullNumberRequestedReviewersResponse {
    ///
    users: Vec<Value>,
    ///
    teams: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoPullspullNumberRequestedReviewersResponse {
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347
    url: String,
    /// * example - 1
    id: i64,
    /// * example - MDExOlB1bGxSZXF1ZXN0MQ==
    node_id: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347
    html_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.diff
    diff_url: String,
    /// * example - https://github.com/octocat/Hello-World/pull/1347.patch
    patch_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347
    issue_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/commits
    commits_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/1347/comments
    review_comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/comments{/number}
    review_comment_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/issues/1347/comments
    comments_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/statuses/6dcb09b5b57875f334f61aebed695e2e4193db5e
    statuses_url: String,
    /// * example - 1347
    number: i64,
    /// * example - open
    state: String,
    /// * example - true
    locked: bool,
    /// * example - new-feature
    title: String,
    /// Simple User
    user: Value,
    /// * example - Please pull these awesome changes
    body: String,
    ///
    labels: Vec<Value>,
    /// A collection of related issues and pull requests.
    milestone: Value,
    /// * example - too heated
    active_lock_reason: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    closed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    merged_at: String,
    /// * example - e5bd3914e2e596debea16f433f57875b5b90bcd6
    merge_commit_sha: String,
    /// Simple User
    assignee: Value,
    ///
    assignees: Vec<Value>,
    ///
    requested_reviewers: Vec<Value>,
    ///
    requested_teams: Vec<Value>,
    ///
    head: Value,
    ///
    base: Value,
    ///
    _links: Value,
    /// * example - OWNER
    author_association: String,
    /// The status of auto merging a pull request.
    auto_merge: Value,
    /// Indicates whether or not the pull request is a draft.
    draft: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoPullspullNumberReviewsResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoPullspullNumberReviewsreviewIdResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoPullspullNumberReviewsreviewIdResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteReposownerrepoPullspullNumberReviewsreviewIdResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoPullspullNumberReviewsreviewIdDismissalsResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoPullspullNumberReviewsreviewIdEventsResponse {
    /// * example - 42
    id: i64,
    /// * example - MDE3OlB1bGxSZXF1ZXN0UmV2aWV3ODA=
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - This looks great.
    body: String,
    /// * example - CHANGES_REQUESTED
    state: String,
    /// * example - https://github.com/octocat/Hello-World/pull/12#pullrequestreview-80
    html_url: String,
    /// * example - https://api.github.com/repos/octocat/Hello-World/pulls/12
    pull_request_url: String,
    ///
    _links: Value,
    ///
    submitted_at: String,
    /// * example - 54bb654c9e6025347f57900a4a5c2313a96b8035
    commit_id: String,
    ///
    body_html: String,
    ///
    body_text: String,
    /// * example - OWNER
    author_association: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReadmeResponse {
    ///
    #[serde(rename = "type")]
    atype: String,
    ///
    encoding: String,
    ///
    size: i64,
    ///
    name: String,
    ///
    path: String,
    ///
    content: String,
    ///
    sha: String,
    ///
    url: String,
    ///
    git_url: String,
    ///
    html_url: String,
    ///
    download_url: String,
    ///
    _links: Value,
    /// * example - "actual/actual.md"
    target: String,
    /// * example - "git://example.com/defunkt/dotjs.git"
    submodule_git_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReadmedirResponse {
    ///
    #[serde(rename = "type")]
    atype: String,
    ///
    encoding: String,
    ///
    size: i64,
    ///
    name: String,
    ///
    path: String,
    ///
    content: String,
    ///
    sha: String,
    ///
    url: String,
    ///
    git_url: String,
    ///
    html_url: String,
    ///
    download_url: String,
    ///
    _links: Value,
    /// * example - "actual/actual.md"
    target: String,
    /// * example - "git://example.com/defunkt/dotjs.git"
    submodule_git_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReleasesAssetsassetIdResponse {
    ///
    url: String,
    ///
    browser_download_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - Team Environment
    name: String,
    ///
    label: String,
    /// State of the release asset.
    state: String,
    ///
    content_type: String,
    ///
    size: i64,
    ///
    download_count: i64,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Simple User
    uploader: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoReleasesAssetsassetIdResponse {
    ///
    url: String,
    ///
    browser_download_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - Team Environment
    name: String,
    ///
    label: String,
    /// State of the release asset.
    state: String,
    ///
    content_type: String,
    ///
    size: i64,
    ///
    download_count: i64,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Simple User
    uploader: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoReleasesGenerateNotesResponse {
    /// * example - Release v1.0.0 is now available!
    name: String,
    /// The generated body describing the contents of the release supporting markdown formatting
    body: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReleasesLatestResponse {
    ///
    url: String,
    ///
    html_url: String,
    ///
    assets_url: String,
    ///
    upload_url: String,
    ///
    tarball_url: String,
    ///
    zipball_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - v1.0.0
    tag_name: String,
    /// * example - master
    target_commitish: String,
    ///
    name: String,
    ///
    body: String,
    /// true to create a draft (unpublished) release, false to create a published one.
    draft: bool,
    /// Whether to identify the release as a prerelease or a full release.
    prerelease: bool,
    ///
    created_at: String,
    ///
    published_at: String,
    /// Simple User
    author: Value,
    ///
    assets: Vec<Value>,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    mentions_count: i64,
    /// The URL of the release discussion.
    discussion_url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReleasesTagstagResponse {
    ///
    url: String,
    ///
    html_url: String,
    ///
    assets_url: String,
    ///
    upload_url: String,
    ///
    tarball_url: String,
    ///
    zipball_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - v1.0.0
    tag_name: String,
    /// * example - master
    target_commitish: String,
    ///
    name: String,
    ///
    body: String,
    /// true to create a draft (unpublished) release, false to create a published one.
    draft: bool,
    /// Whether to identify the release as a prerelease or a full release.
    prerelease: bool,
    ///
    created_at: String,
    ///
    published_at: String,
    /// Simple User
    author: Value,
    ///
    assets: Vec<Value>,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    mentions_count: i64,
    /// The URL of the release discussion.
    discussion_url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoReleasesreleaseIdResponse {
    ///
    url: String,
    ///
    html_url: String,
    ///
    assets_url: String,
    ///
    upload_url: String,
    ///
    tarball_url: String,
    ///
    zipball_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - v1.0.0
    tag_name: String,
    /// * example - master
    target_commitish: String,
    ///
    name: String,
    ///
    body: String,
    /// true to create a draft (unpublished) release, false to create a published one.
    draft: bool,
    /// Whether to identify the release as a prerelease or a full release.
    prerelease: bool,
    ///
    created_at: String,
    ///
    published_at: String,
    /// Simple User
    author: Value,
    ///
    assets: Vec<Value>,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    mentions_count: i64,
    /// The URL of the release discussion.
    discussion_url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoReleasesreleaseIdResponse {
    ///
    url: String,
    ///
    html_url: String,
    ///
    assets_url: String,
    ///
    upload_url: String,
    ///
    tarball_url: String,
    ///
    zipball_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    /// * example - v1.0.0
    tag_name: String,
    /// * example - master
    target_commitish: String,
    ///
    name: String,
    ///
    body: String,
    /// true to create a draft (unpublished) release, false to create a published one.
    draft: bool,
    /// Whether to identify the release as a prerelease or a full release.
    prerelease: bool,
    ///
    created_at: String,
    ///
    published_at: String,
    /// Simple User
    author: Value,
    ///
    assets: Vec<Value>,
    ///
    body_html: String,
    ///
    body_text: String,
    ///
    mentions_count: i64,
    /// The URL of the release discussion.
    discussion_url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostReposownerrepoReleasesreleaseIdReactionsResponse {
    /// * example - 1
    id: i64,
    /// * example - MDg6UmVhY3Rpb24x
    node_id: String,
    /// Simple User
    user: Value,
    /// * example - heart
    content: String,
    /// * example - 2016-05-20T20:09:31Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoSecretScanningAlertsalertNumberResponse {
    /// The security alert number.
    number: i64,
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    created_at: String,
    /// The REST API URL of the alert resource.
    url: String,
    /// The GitHub URL of the alert resource.
    html_url: String,
    /// The REST API URL of the code locations for this alert.
    locations_url: String,
    /// Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
    state: String,
    /// **Required when the `state` is `resolved`.** The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
    resolution: String,
    /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    resolved_at: String,
    /// Simple User
    resolved_by: Value,
    /// The type of secret that secret scanning detected.
    secret_type: String,
    /// The secret that was detected.
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchReposownerrepoSecretScanningAlertsalertNumberResponse {
    /// The security alert number.
    number: i64,
    /// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    created_at: String,
    /// The REST API URL of the alert resource.
    url: String,
    /// The GitHub URL of the alert resource.
    html_url: String,
    /// The REST API URL of the code locations for this alert.
    locations_url: String,
    /// Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
    state: String,
    /// **Required when the `state` is `resolved`.** The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
    resolution: String,
    /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    resolved_at: String,
    /// Simple User
    resolved_by: Value,
    /// The type of secret that secret scanning detected.
    secret_type: String,
    /// The secret that was detected.
    secret: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoStatsParticipationResponse {
    ///
    all: Vec<i64>,
    ///
    owner: Vec<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoSubscriptionResponse {
    /// * example - true
    subscribed: bool,
    /// Determines if all notifications should be blocked from this repository.
    ignored: bool,
    ///
    reason: String,
    /// * example - 2012-10-06T21:34:12Z
    created_at: String,
    /// * example - https://api.github.com/repos/octocat/example/subscription
    url: String,
    /// * example - https://api.github.com/repos/octocat/example
    repository_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoSubscriptionResponse {
    /// * example - true
    subscribed: bool,
    /// Determines if all notifications should be blocked from this repository.
    ignored: bool,
    ///
    reason: String,
    /// * example - 2012-10-06T21:34:12Z
    created_at: String,
    /// * example - https://api.github.com/repos/octocat/example/subscription
    url: String,
    /// * example - https://api.github.com/repos/octocat/example
    repository_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoTopicsResponse {
    ///
    names: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutReposownerrepoTopicsResponse {
    ///
    names: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoTrafficClonesResponse {
    /// * example - 173
    count: i64,
    /// * example - 128
    uniques: i64,
    ///
    clones: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetReposownerrepoTrafficViewsResponse {
    /// * example - 14850
    count: i64,
    /// * example - 3782
    uniques: i64,
    ///
    views: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsResponse {
    ///
    total_count: i64,
    ///
    secrets: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKeyResponse {
    /// * example - 1234567
    key_id: String,
    /// * example - hBT5WZEj8ZoOv6TYJsfWq7MxTEQopZO5/IT3ZCVQPzs=
    key: String,
    /// * example - 2
    id: i64,
    /// * example - https://api.github.com/user/keys/2
    url: String,
    /// * example - ssh-rsa AAAAB3NzaC1yc2EAAA
    title: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretNameResponse {
    /// * example - SECRET_TOKEN
    name: String,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetScimV2EnterprisesenterpriseGroupsResponse {
    ///
    schemas: Vec<String>,
    ///
    totalResults: f64,
    ///
    itemsPerPage: f64,
    ///
    startIndex: f64,
    ///
    Resources: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetScimV2EnterprisesenterpriseGroupsscimGroupIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    displayName: String,
    ///
    members: Vec<Value>,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutScimV2EnterprisesenterpriseGroupsscimGroupIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    displayName: String,
    ///
    members: Vec<Value>,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchScimV2EnterprisesenterpriseGroupsscimGroupIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    displayName: String,
    ///
    members: Vec<Value>,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetScimV2EnterprisesenterpriseUsersResponse {
    ///
    schemas: Vec<String>,
    ///
    totalResults: f64,
    ///
    itemsPerPage: f64,
    ///
    startIndex: f64,
    ///
    Resources: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetScimV2EnterprisesenterpriseUsersscimUserIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    userName: String,
    ///
    name: Value,
    ///
    emails: Vec<Value>,
    ///
    groups: Vec<Value>,
    ///
    active: bool,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutScimV2EnterprisesenterpriseUsersscimUserIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    userName: String,
    ///
    name: Value,
    ///
    emails: Vec<Value>,
    ///
    groups: Vec<Value>,
    ///
    active: bool,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchScimV2EnterprisesenterpriseUsersscimUserIdResponse {
    ///
    schemas: Vec<String>,
    ///
    id: String,
    ///
    externalId: String,
    ///
    userName: String,
    ///
    name: Value,
    ///
    emails: Vec<Value>,
    ///
    groups: Vec<Value>,
    ///
    active: bool,
    ///
    meta: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchCodeResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchCommitsResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchIssuesResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchLabelsResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchRepositoriesResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchTopicsResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSearchUsersResponse {
    ///
    total_count: i64,
    ///
    incomplete_results: bool,
    ///
    items: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdResponse {
    /// * example - 42
    id: i64,
    /// * example - MDQ6VGVhbTE=
    node_id: String,
    /// * example - https://api.github.com/organizations/1/team/1
    url: String,
    /// * example - https://github.com/orgs/rails/teams/core
    html_url: String,
    /// * example - Developers
    name: String,
    /// * example - justice-league
    slug: String,
    /// * example - A great team.
    description: String,
    /// * example - closed
    privacy: String,
    /// * example - push
    permission: String,
    /// * example - https://api.github.com/organizations/1/team/1/members{/member}
    members_url: String,
    /// * example - https://api.github.com/organizations/1/team/1/repos
    repositories_url: String,
    /// Groups of organization members that gives permissions on specified repositories.
    parent: Value,
    /// * example - 3
    members_count: i64,
    /// * example - 10
    repos_count: i64,
    /// * example - 2017-07-14T16:53:42Z
    created_at: String,
    /// * example - 2017-08-17T12:37:15Z
    updated_at: String,
    /// Organization Full
    organization: Value,
    /// * example - uid=example,ou=users,dc=github,dc=com
    ldap_dn: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchTeamsteamIdResponse {
    /// * example - 42
    id: i64,
    /// * example - MDQ6VGVhbTE=
    node_id: String,
    /// * example - https://api.github.com/organizations/1/team/1
    url: String,
    /// * example - https://github.com/orgs/rails/teams/core
    html_url: String,
    /// * example - Developers
    name: String,
    /// * example - justice-league
    slug: String,
    /// * example - A great team.
    description: String,
    /// * example - closed
    privacy: String,
    /// * example - push
    permission: String,
    /// * example - https://api.github.com/organizations/1/team/1/members{/member}
    members_url: String,
    /// * example - https://api.github.com/organizations/1/team/1/repos
    repositories_url: String,
    /// Groups of organization members that gives permissions on specified repositories.
    parent: Value,
    /// * example - 3
    members_count: i64,
    /// * example - 10
    repos_count: i64,
    /// * example - 2017-07-14T16:53:42Z
    created_at: String,
    /// * example - 2017-08-17T12:37:15Z
    updated_at: String,
    /// Organization Full
    organization: Value,
    /// * example - uid=example,ou=users,dc=github,dc=com
    ldap_dn: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdDiscussionsdiscussionNumberResponse {
    /// Simple User
    author: Value,
    /// * example - Please suggest improvements to our workflow in comments.
    body: String,
    /// * example - <p>Hi! This is an area for us to collaborate as a team</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    ///
    comments_count: i64,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1/comments
    comments_url: String,
    /// * example - 2018-01-25T18:56:31Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1
    html_url: String,
    /// * example - MDE0OlRlYW1EaXNjdXNzaW9uMQ==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - true
    pinned: bool,
    /// * example - true
    private: bool,
    /// * example - https://api.github.com/organizations/1/team/2343027
    team_url: String,
    /// * example - How can we improve our workflow?
    title: String,
    /// * example - 2018-01-25T18:56:31Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchTeamsteamIdDiscussionsdiscussionNumberResponse {
    /// Simple User
    author: Value,
    /// * example - Please suggest improvements to our workflow in comments.
    body: String,
    /// * example - <p>Hi! This is an area for us to collaborate as a team</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    ///
    comments_count: i64,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1/comments
    comments_url: String,
    /// * example - 2018-01-25T18:56:31Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1
    html_url: String,
    /// * example - MDE0OlRlYW1EaXNjdXNzaW9uMQ==
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - true
    pinned: bool,
    /// * example - true
    private: bool,
    /// * example - https://api.github.com/organizations/1/team/2343027
    team_url: String,
    /// * example - How can we improve our workflow?
    title: String,
    /// * example - 2018-01-25T18:56:31Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2343027/discussions/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse {
    /// Simple User
    author: Value,
    /// * example - I agree with this suggestion.
    body: String,
    /// * example - <p>Do you like apples?</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    /// * example - 2018-01-15T23:53:58Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1
    discussion_url: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1/comments/1
    html_url: String,
    /// * example - MDIxOlRlYW1EaXNjdXNzaW9uQ29tbWVudDE=
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - 2018-01-15T23:53:58Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1/comments/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse {
    /// Simple User
    author: Value,
    /// * example - I agree with this suggestion.
    body: String,
    /// * example - <p>Do you like apples?</p>
    body_html: String,
    /// * example - 0307116bbf7ced493b8d8a346c650b71
    body_version: String,
    /// * example - 2018-01-15T23:53:58Z
    created_at: String,
    ///
    last_edited_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1
    discussion_url: String,
    /// * example - https://github.com/orgs/github/teams/justice-league/discussions/1/comments/1
    html_url: String,
    /// * example - MDIxOlRlYW1EaXNjdXNzaW9uQ29tbWVudDE=
    node_id: String,
    /// * example - 42
    number: i64,
    /// * example - 2018-01-15T23:53:58Z
    updated_at: String,
    /// * example - https://api.github.com/organizations/1/team/2403582/discussions/1/comments/1
    url: String,
    ///
    reactions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdMembershipsusernameResponse {
    ///
    url: String,
    /// * example - member
    role: String,
    /// The state of the user's membership in the team.
    state: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutTeamsteamIdMembershipsusernameResponse {
    ///
    url: String,
    /// * example - member
    role: String,
    /// The state of the user's membership in the team.
    state: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdProjectsprojectIdResponse {
    ///
    owner_url: String,
    ///
    url: String,
    ///
    html_url: String,
    ///
    columns_url: String,
    ///
    id: i64,
    ///
    node_id: String,
    ///
    name: String,
    ///
    body: String,
    ///
    number: i64,
    ///
    state: String,
    /// Simple User
    creator: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// The organization permission for this project. Only present when owner is an organization.
    organization_permission: String,
    /// Whether the project is private or not. Only present when owner is an organization.
    private: bool,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdReposownerrepoResponse {
    /// * example - 42
    id: i64,
    /// * example - MDEwOlJlcG9zaXRvcnkxMjk2MjY5
    node_id: String,
    /// * example - Team Environment
    name: String,
    /// * example - octocat/Hello-World
    full_name: String,
    /// License Simple
    license: Value,
    ///
    forks: i64,
    ///
    permissions: Value,
    /// * example - admin
    role_name: String,
    /// Simple User
    owner: Value,
    /// Whether the repository is private or public.
    private: bool,
    /// * example - https://github.com/octocat/Hello-World
    html_url: String,
    /// * example - This your first repo!
    description: String,
    ///
    fork: bool,
    /// * example - https://api.github.com/repos/octocat/Hello-World
    url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/{archive_format}{/ref}
    archive_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/assignees{/user}
    assignees_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/blobs{/sha}
    blobs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/branches{/branch}
    branches_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/collaborators{/collaborator}
    collaborators_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/comments{/number}
    comments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/commits{/sha}
    commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/compare/{base}...{head}
    compare_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contents/{+path}
    contents_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/contributors
    contributors_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/deployments
    deployments_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/downloads
    downloads_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/events
    events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/forks
    forks_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/commits{/sha}
    git_commits_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/refs{/sha}
    git_refs_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/tags{/sha}
    git_tags_url: String,
    /// * example - git:github.com/octocat/Hello-World.git
    git_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/comments{/number}
    issue_comment_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues/events{/number}
    issue_events_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/issues{/number}
    issues_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/keys{/key_id}
    keys_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/labels{/name}
    labels_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/languages
    languages_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/merges
    merges_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/milestones{/number}
    milestones_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/notifications{?since,all,participating}
    notifications_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/pulls{/number}
    pulls_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/releases{/id}
    releases_url: String,
    /// * example - git@github.com:octocat/Hello-World.git
    ssh_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/stargazers
    stargazers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/statuses/{sha}
    statuses_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscribers
    subscribers_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/subscription
    subscription_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/tags
    tags_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/teams
    teams_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/git/trees{/sha}
    trees_url: String,
    /// * example - https://github.com/octocat/Hello-World.git
    clone_url: String,
    /// * example - git:git.example.com/octocat/Hello-World
    mirror_url: String,
    /// * example - http://api.github.com/repos/octocat/Hello-World/hooks
    hooks_url: String,
    /// * example - https://svn.github.com/octocat/Hello-World
    svn_url: String,
    /// * example - https://github.com
    homepage: String,
    ///
    language: String,
    /// * example - 9
    forks_count: i64,
    /// * example - 80
    stargazers_count: i64,
    /// * example - 80
    watchers_count: i64,
    /// * example - 108
    size: i64,
    /// * example - master
    default_branch: String,
    ///
    open_issues_count: i64,
    /// * example - true
    is_template: bool,
    ///
    topics: Vec<String>,
    /// * example - true
    has_issues: bool,
    /// * example - true
    has_projects: bool,
    /// * example - true
    has_wiki: bool,
    ///
    has_pages: bool,
    /// * example - true
    has_downloads: bool,
    /// Whether the repository is archived.
    archived: bool,
    /// Returns whether or not this repository disabled.
    disabled: bool,
    /// The repository visibility: public, private, or internal.
    visibility: String,
    /// * example - 2011-01-26T19:06:43Z
    pushed_at: String,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:14:43Z
    updated_at: String,
    /// * example - true
    allow_rebase_merge: bool,
    /// A git repository
    template_repository: Value,
    ///
    temp_clone_token: String,
    /// * example - true
    allow_squash_merge: bool,
    /// Whether to allow Auto-merge to be used on pull requests.
    allow_auto_merge: bool,
    /// Whether to delete head branches when pull requests are merged
    delete_branch_on_merge: bool,
    /// * example - true
    allow_merge_commit: bool,
    /// Whether to allow forking this repo
    allow_forking: bool,
    ///
    subscribers_count: i64,
    ///
    network_count: i64,
    ///
    open_issues: i64,
    ///
    watchers: i64,
    ///
    master_branch: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTeamsteamIdTeamSyncGroupMappingsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchTeamsteamIdTeamSyncGroupMappingsResponse {
    /// * example - [object Object],[object Object]
    groups: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchUserResponse {
    /// * example - octocat
    login: String,
    /// * example - 1
    id: i64,
    /// * example - MDQ6VXNlcjE=
    node_id: String,
    /// * example - https://github.com/images/error/octocat_happy.gif
    avatar_url: String,
    /// * example - 41d064eb2195891e12d0413f63227ea7
    gravatar_id: String,
    /// * example - https://api.github.com/users/octocat
    url: String,
    /// * example - https://github.com/octocat
    html_url: String,
    /// * example - https://api.github.com/users/octocat/followers
    followers_url: String,
    /// * example - https://api.github.com/users/octocat/following{/other_user}
    following_url: String,
    /// * example - https://api.github.com/users/octocat/gists{/gist_id}
    gists_url: String,
    /// * example - https://api.github.com/users/octocat/starred{/owner}{/repo}
    starred_url: String,
    /// * example - https://api.github.com/users/octocat/subscriptions
    subscriptions_url: String,
    /// * example - https://api.github.com/users/octocat/orgs
    organizations_url: String,
    /// * example - https://api.github.com/users/octocat/repos
    repos_url: String,
    /// * example - https://api.github.com/users/octocat/events{/privacy}
    events_url: String,
    /// * example - https://api.github.com/users/octocat/received_events
    received_events_url: String,
    /// * example - User
    #[serde(rename = "type")]
    atype: String,
    ///
    site_admin: bool,
    /// * example - monalisa octocat
    name: String,
    /// * example - GitHub
    company: String,
    /// * example - https://github.com/blog
    blog: String,
    /// * example - San Francisco
    location: String,
    /// * example - octocat@github.com
    email: String,
    ///
    hireable: bool,
    /// * example - There once was...
    bio: String,
    /// * example - monalisa
    twitter_username: String,
    /// * example - 2
    public_repos: i64,
    /// * example - 1
    public_gists: i64,
    /// * example - 20
    followers: i64,
    ///
    following: i64,
    /// * example - 2008-01-14T04:33:35Z
    created_at: String,
    /// * example - 2008-01-14T04:33:35Z
    updated_at: String,
    /// * example - 81
    private_gists: i64,
    /// * example - 100
    total_private_repos: i64,
    /// * example - 100
    owned_private_repos: i64,
    /// * example - 10000
    disk_usage: i64,
    /// * example - 8
    collaborators: i64,
    /// * example - true
    two_factor_authentication: bool,
    ///
    plan: Value,
    ///
    suspended_at: String,
    ///
    business_plus: bool,
    ///
    ldap_dn: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacesResponse {
    ///
    total_count: i64,
    ///
    codespaces: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacesSecretsResponse {
    ///
    total_count: i64,
    ///
    secrets: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacesSecretsPublicKeyResponse {
    /// * example - 1234567
    key_id: String,
    /// * example - hBT5WZEj8ZoOv6TYJsfWq7MxTEQopZO5/IT3ZCVQPzs=
    key: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacesSecretssecretNameResponse {
    /// * example - SECRET_NAME
    name: String,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// Visibility of a secret
    visibility: String,
    /// * example - https://api.github.com/user/secrets/SECRET_NAME/repositories
    selected_repositories_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacesSecretssecretNameRepositoriesResponse {
    ///
    total_count: i64,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacescodespaceNameResponse {
    /// * example - 1
    id: i64,
    /// * example - monalisa-octocat-hello-world-g4wpq6h95q
    name: String,
    /// * example - 26a7c758-7299-4a73-b978-5a92a7ae98a0
    environment_id: String,
    /// Simple User
    owner: Value,
    /// Simple User
    billable_owner: Value,
    /// Minimal Repository
    repository: Value,
    /// A description of the machine powering a codespace.
    machine: Value,
    /// Whether the codespace was created from a prebuild.
    prebuild: bool,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    last_used_at: String,
    /// * example - Available
    state: String,
    /// API URL for this codespace.
    url: String,
    /// Details about the codespace's git repository.
    git_status: Value,
    /// * example - WestUs2
    location: String,
    /// * example - 60
    idle_timeout_minutes: i64,
    /// URL to access this codespace on the web.
    web_url: String,
    /// API URL to access available alternate machine types for this codespace.
    machines_url: String,
    /// API URL to start this codespace.
    start_url: String,
    /// API URL to stop this codespace.
    stop_url: String,
    /// API URL for the Pull Request associated with this codespace, if any.
    pulls_url: String,
    ///
    recent_folders: Vec<String>,
    ///
    runtime_constraints: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchUserCodespacescodespaceNameResponse {
    /// * example - 1
    id: i64,
    /// * example - monalisa-octocat-hello-world-g4wpq6h95q
    name: String,
    /// * example - 26a7c758-7299-4a73-b978-5a92a7ae98a0
    environment_id: String,
    /// Simple User
    owner: Value,
    /// Simple User
    billable_owner: Value,
    /// Minimal Repository
    repository: Value,
    /// A description of the machine powering a codespace.
    machine: Value,
    /// Whether the codespace was created from a prebuild.
    prebuild: bool,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    last_used_at: String,
    /// * example - Available
    state: String,
    /// API URL for this codespace.
    url: String,
    /// Details about the codespace's git repository.
    git_status: Value,
    /// * example - WestUs2
    location: String,
    /// * example - 60
    idle_timeout_minutes: i64,
    /// URL to access this codespace on the web.
    web_url: String,
    /// API URL to access available alternate machine types for this codespace.
    machines_url: String,
    /// API URL to start this codespace.
    start_url: String,
    /// API URL to stop this codespace.
    stop_url: String,
    /// API URL for the Pull Request associated with this codespace, if any.
    pulls_url: String,
    ///
    recent_folders: Vec<String>,
    ///
    runtime_constraints: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserCodespacescodespaceNameMachinesResponse {
    ///
    total_count: i64,
    ///
    machines: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUserCodespacescodespaceNameStartResponse {
    /// * example - 1
    id: i64,
    /// * example - monalisa-octocat-hello-world-g4wpq6h95q
    name: String,
    /// * example - 26a7c758-7299-4a73-b978-5a92a7ae98a0
    environment_id: String,
    /// Simple User
    owner: Value,
    /// Simple User
    billable_owner: Value,
    /// Minimal Repository
    repository: Value,
    /// A description of the machine powering a codespace.
    machine: Value,
    /// Whether the codespace was created from a prebuild.
    prebuild: bool,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    last_used_at: String,
    /// * example - Available
    state: String,
    /// API URL for this codespace.
    url: String,
    /// Details about the codespace's git repository.
    git_status: Value,
    /// * example - WestUs2
    location: String,
    /// * example - 60
    idle_timeout_minutes: i64,
    /// URL to access this codespace on the web.
    web_url: String,
    /// API URL to access available alternate machine types for this codespace.
    machines_url: String,
    /// API URL to start this codespace.
    start_url: String,
    /// API URL to stop this codespace.
    stop_url: String,
    /// API URL for the Pull Request associated with this codespace, if any.
    pulls_url: String,
    ///
    recent_folders: Vec<String>,
    ///
    runtime_constraints: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUserCodespacescodespaceNameStopResponse {
    /// * example - 1
    id: i64,
    /// * example - monalisa-octocat-hello-world-g4wpq6h95q
    name: String,
    /// * example - 26a7c758-7299-4a73-b978-5a92a7ae98a0
    environment_id: String,
    /// Simple User
    owner: Value,
    /// Simple User
    billable_owner: Value,
    /// Minimal Repository
    repository: Value,
    /// A description of the machine powering a codespace.
    machine: Value,
    /// Whether the codespace was created from a prebuild.
    prebuild: bool,
    /// * example - 2011-01-26T19:01:12Z
    created_at: String,
    /// * example - 2011-01-26T19:01:12Z
    updated_at: String,
    /// * example - 2011-01-26T19:01:12Z
    last_used_at: String,
    /// * example - Available
    state: String,
    /// API URL for this codespace.
    url: String,
    /// Details about the codespace's git repository.
    git_status: Value,
    /// * example - WestUs2
    location: String,
    /// * example - 60
    idle_timeout_minutes: i64,
    /// URL to access this codespace on the web.
    web_url: String,
    /// API URL to access available alternate machine types for this codespace.
    machines_url: String,
    /// API URL to start this codespace.
    start_url: String,
    /// API URL to stop this codespace.
    stop_url: String,
    /// API URL for the Pull Request associated with this codespace, if any.
    pulls_url: String,
    ///
    recent_folders: Vec<String>,
    ///
    runtime_constraints: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserGpgKeysgpgKeyIdResponse {
    /// * example - 3
    id: i64,
    ///
    primary_key_id: i64,
    /// * example - 3262EFF25BA0D270
    key_id: String,
    /// * example - xsBNBFayYZ...
    public_key: String,
    /// * example - [object Object]
    emails: Vec<Value>,
    /// * example - [object Object]
    subkeys: Vec<Value>,
    /// * example - true
    can_sign: bool,
    ///
    can_encrypt_comms: bool,
    ///
    can_encrypt_storage: bool,
    /// * example - true
    can_certify: bool,
    /// * example - 2016-03-24T11:31:04-06:00
    created_at: String,
    ///
    expires_at: String,
    ///
    raw_key: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserInstallationsResponse {
    ///
    total_count: i64,
    ///
    installations: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserInstallationsinstallationIdRepositoriesResponse {
    ///
    total_count: i64,
    ///
    repository_selection: String,
    ///
    repositories: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutUserInteractionLimitsResponse {
    /// * example - collaborators_only
    limit: String,
    /// * example - repository
    origin: String,
    /// * example - 2018-08-17T04:18:39Z
    expires_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserKeyskeyIdResponse {
    ///
    key: String,
    ///
    id: i64,
    ///
    url: String,
    ///
    title: String,
    ///
    created_at: String,
    ///
    verified: bool,
    ///
    read_only: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserMembershipsOrgsorgResponse {
    /// * example - https://api.github.com/orgs/octocat/memberships/defunkt
    url: String,
    /// * example - active
    state: String,
    /// * example - admin
    role: String,
    /// * example - https://api.github.com/orgs/octocat
    organization_url: String,
    /// Organization Simple
    organization: Value,
    /// Simple User
    user: Value,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatchUserMembershipsOrgsorgResponse {
    /// * example - https://api.github.com/orgs/octocat/memberships/defunkt
    url: String,
    /// * example - active
    state: String,
    /// * example - admin
    role: String,
    /// * example - https://api.github.com/orgs/octocat
    organization_url: String,
    /// Organization Simple
    organization: Value,
    /// Simple User
    user: Value,
    ///
    permissions: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserMigrationsmigrationIdResponse {
    /// * example - 79
    id: i64,
    /// Simple User
    owner: Value,
    /// * example - 0b989ba4-242f-11e5-81e1-c7b6966d2516
    guid: String,
    /// * example - pending
    state: String,
    /// * example - true
    lock_repositories: bool,
    ///
    exclude_metadata: bool,
    ///
    exclude_git_data: bool,
    ///
    exclude_attachments: bool,
    ///
    exclude_releases: bool,
    ///
    exclude_owner_projects: bool,
    ///
    repositories: Vec<Value>,
    /// * example - https://api.github.com/orgs/octo-org/migrations/79
    url: String,
    /// * example - 2015-07-06T15:33:38-07:00
    created_at: String,
    /// * example - 2015-07-06T15:33:38-07:00
    updated_at: String,
    ///
    node_id: String,
    ///
    archive_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserPackagespackageTypepackageNameResponse {
    /// * example - 1
    id: i64,
    /// * example - super-linter
    name: String,
    /// * example - docker
    package_type: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    html_url: String,
    /// * example - 1
    version_count: i64,
    /// * example - private
    visibility: String,
    /// Simple User
    owner: Value,
    /// Minimal Repository
    repository: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserPackagespackageTypepackageNameVersionspackageVersionIdResponse {
    /// * example - 1
    id: i64,
    /// * example - latest
    name: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter/versions/786068
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    package_html_url: String,
    /// * example - https://github.com/orgs/github/packages/container/super-linter/786068
    html_url: String,
    /// * example - MIT
    license: String,
    ///
    description: String,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// * example - 2014-03-03T18:58:10Z
    deleted_at: String,
    ///
    metadata: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernameHovercardResponse {
    ///
    contexts: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernameInstallationResponse {
    /// * example - 1
    id: i64,
    /// Describe whether all repositories have been selected or there's a selection involved
    repository_selection: String,
    /// * example - https://api.github.com/installations/1/access_tokens
    access_tokens_url: String,
    /// * example - https://api.github.com/installation/repositories
    repositories_url: String,
    /// * example - https://github.com/organizations/github/settings/installations/1
    html_url: String,
    /// * example - 1
    app_id: i64,
    /// The ID of the user or organization this token is being scoped to.
    target_id: i64,
    /// * example - Organization
    target_type: String,
    /// * example - [object Object]
    permissions: Value,
    ///
    events: Vec<String>,
    ///
    created_at: String,
    ///
    updated_at: String,
    /// * example - config.yaml
    single_file_name: String,
    /// * example - true
    has_multiple_single_files: bool,
    /// * example - config.yml,.github/issue_TEMPLATE.md
    single_file_paths: Vec<String>,
    /// * example - github-actions
    app_slug: String,
    /// Simple User
    suspended_by: Value,
    ///
    suspended_at: String,
    /// * example - "test_13f1e99741e3e004@d7e1eb0bc0a1ba12.com"
    contact_email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernamePackagespackageTypepackageNameResponse {
    /// * example - 1
    id: i64,
    /// * example - super-linter
    name: String,
    /// * example - docker
    package_type: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    html_url: String,
    /// * example - 1
    version_count: i64,
    /// * example - private
    visibility: String,
    /// Simple User
    owner: Value,
    /// Minimal Repository
    repository: Value,
    ///
    created_at: String,
    ///
    updated_at: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdResponse {
    /// * example - 1
    id: i64,
    /// * example - latest
    name: String,
    /// * example - https://api.github.com/orgs/github/packages/container/super-linter/versions/786068
    url: String,
    /// * example - https://github.com/orgs/github/packages/container/package/super-linter
    package_html_url: String,
    /// * example - https://github.com/orgs/github/packages/container/super-linter/786068
    html_url: String,
    /// * example - MIT
    license: String,
    ///
    description: String,
    /// * example - 2011-04-10T20:09:31Z
    created_at: String,
    /// * example - 2014-03-03T18:58:10Z
    updated_at: String,
    /// * example - 2014-03-03T18:58:10Z
    deleted_at: String,
    ///
    metadata: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernameSettingsBillingActionsResponse {
    /// The sum of the free and paid GitHub Actions minutes used.
    total_minutes_used: i64,
    /// The total paid GitHub Actions minutes used.
    total_paid_minutes_used: i64,
    /// The amount of free GitHub Actions minutes available.
    included_minutes: i64,
    ///
    minutes_used_breakdown: Value,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernameSettingsBillingPackagesResponse {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    total_gigabytes_bandwidth_used: i64,
    /// Total paid storage space (GB) for GitHuub Packages.
    total_paid_gigabytes_bandwidth_used: i64,
    /// Free storage space (GB) for GitHub Packages.
    included_gigabytes_bandwidth: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersusernameSettingsBillingSharedStorageResponse {
    /// Numbers of days left in billing cycle.
    days_left_in_billing_cycle: i64,
    /// Estimated storage space (GB) used in billing cycle.
    estimated_paid_storage_for_month: i64,
    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    estimated_storage_for_month: i64,
}
